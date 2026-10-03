// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Cluster;
use anyhow::{Context, Result, ensure};
use std::process::Command;

/// Preserve all default-build and missing-manifest rejections with fixed phase context.
pub(crate) fn verify_default_build_guards(cluster: &Cluster) -> Result<()> {
    let normal = std::env::var_os("EVE_VALIDATOR_NORMAL_BINARY")
        .context("B3_TC07_NORMAL_BINARY_REQUIRED")?;
    let command = crate::support::process::node_command(cluster, 0, "init-dev", "");
    let arguments: Vec<_> = command
        .get_args()
        .map(|argument| argument.to_owned())
        .collect();
    let rejected = Command::new(&normal)
        .args(&arguments)
        .output()
        .context("B3_TC07_DEFAULT_REJECTION")?;
    ensure!(
        !rejected.status.success(),
        "B3_TC07_DEFAULT_ACCEPTED: default build accepted temporary adapter genesis"
    );
    let flag = arguments
        .iter()
        .position(|argument| argument == "--acceptance-fixture")
        .context("B3_TC07_FIXTURE_FLAG_REQUIRED")?;
    let without_fixture: Vec<_> = arguments
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != flag && *index != flag + 1)
        .map(|(_, argument)| argument)
        .collect();
    let normal_without = Command::new(&normal)
        .args(&without_fixture)
        .output()
        .context("B3_TC07_NORMAL_MISSING_MANIFEST")?;
    ensure!(
        !normal_without.status.success(),
        "B3_TC07_NORMAL_MANIFEST_ACCEPTED: marked genesis ran without acceptance manifest"
    );
    let acceptance_without = Command::new(&cluster.binary)
        .args(&without_fixture)
        .output()
        .context("B3_TC07_ACCEPTANCE_MISSING_MANIFEST")?;
    ensure!(
        !acceptance_without.status.success(),
        "B3_TC07_ACCEPTANCE_MANIFEST_ACCEPTED: marked genesis ran without acceptance manifest"
    );
    Ok(())
}

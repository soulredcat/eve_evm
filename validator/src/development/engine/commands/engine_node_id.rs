// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::run_engine_command;
use crate::development::{
    config::DevelopmentValidatorConfig,
    engine::{
        home::{acquire_engine_lease, validate_engine_home},
        verification::{validate_engine_namespace, verify_engine_binary},
    },
};
use anyhow::{Result, ensure};
use std::path::Path;

pub(crate) fn engine_node_id(
    config: &DevelopmentValidatorConfig,
    home: &Path,
    expected_sha256: [u8; 32],
) -> Result<String> {
    validate_engine_namespace(config, home)?;
    let _lease = acquire_engine_lease(&config.data)?;
    let binary = verify_engine_binary(config, expected_sha256)?;
    validate_engine_home(home, &binary)?;
    let output = run_engine_command(
        &binary.path,
        &[
            "show-node-id".into(),
            "--home".into(),
            home.as_os_str().to_owned(),
            "--log_level".into(),
            "error".into(),
        ],
        &config.data,
        "node-id",
    )?;
    let identity = std::str::from_utf8(&output)?.trim();
    ensure!(
        identity.len() == 40 && identity.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "native node ID must be exact 20-byte routing identity"
    );
    Ok(identity.to_ascii_lowercase())
}

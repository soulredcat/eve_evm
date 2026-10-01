// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    structure::inspection::resolve_source_path::resolve_source_path,
    verification::types::manifest_types::TestGroup,
};
use anyhow::{Result, ensure};
use std::path::Path;

pub fn load_test_group(root: &Path, path: &str) -> Result<TestGroup> {
    let group: TestGroup =
        toml::from_str(&std::fs::read_to_string(resolve_source_path(root, path)?)?)?;
    ensure!(
        group.version == 1
            && !group.package.trim().is_empty()
            && !group.tests.is_empty()
            && !group.requirements.is_empty(),
        "Empty or unsupported test group {path}"
    );
    let features = group
        .features
        .iter()
        .collect::<std::collections::BTreeSet<_>>();
    ensure!(
        features.len() == group.features.len()
            && group.features.iter().all(|name| name
                .as_bytes()
                .first()
                .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
                && name.len() <= 64
                && name
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))),
        "Invalid or duplicate Cargo feature in test group {path}"
    );
    Ok(group)
}

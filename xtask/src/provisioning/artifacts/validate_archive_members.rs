// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::path::{Component, Path};

/// Validate the complete tar name list before invoking extraction.
pub fn validate_archive_members(names: &str, root: &str) -> Result<()> {
    ensure!(
        names.len() <= 16 * 1024 * 1024,
        "archive member list exceeds limit"
    );
    let mut count = 0;
    for member in names.lines() {
        count += 1;
        ensure!(count <= 100_000, "archive member count exceeds limit");
        ensure!(
            !member.is_empty()
                && !member.contains('\\')
                && !member.bytes().any(|byte| byte.is_ascii_control()),
            "invalid archive member name"
        );
        let path = Path::new(member);
        ensure!(
            path.components()
                .all(|part| matches!(part, Component::Normal(_))),
            "archive contains absolute or traversing member"
        );
        ensure!(
            path.starts_with(root),
            "archive contains an unexpected top-level source"
        );
    }
    ensure!(count != 0, "empty archive member list");
    Ok(())
}

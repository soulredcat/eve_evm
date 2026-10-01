// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    fetch_shanghai_archive::fetch_shanghai_archive,
    load_shanghai_inventory::load_shanghai_inventory,
    types::{ARCHIVE_BYTES, ARCHIVE_SHA256, ARCHIVE_URL, SOURCE_REVISION},
    validate_shanghai_extraction::validate_shanghai_extraction,
};
use crate::{
    provisioning::{
        read_archive_listing, resolve_contained_path, validate_archive_links,
        validate_archive_members,
    },
    verification::{
        commands::require_command_success::require_command_success,
        types::report_types::VerificationReport,
    },
};
use anyhow::{Context, Result, ensure};
use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

/// Prepare the complete pinned subset; only verified data is passed to test children.
pub fn prepare_shanghai_fixtures(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
) -> Result<PathBuf> {
    let pins: toml::Value = toml::from_str(&std::fs::read_to_string(
        root.join("config/test-corpora.toml"),
    )?)?;
    let pin = pins
        .get("ethereum_execution")
        .context("Ethereum execution corpus pin missing")?;
    ensure!(
        pin.get("revision").and_then(toml::Value::as_str) == Some(SOURCE_REVISION)
            && pin.get("sha256").and_then(toml::Value::as_str) == Some(ARCHIVE_SHA256)
            && pin.get("archive").and_then(toml::Value::as_str) == Some(ARCHIVE_URL)
            && pin.get("bytes").and_then(toml::Value::as_integer)
                == i64::try_from(ARCHIVE_BYTES).ok(),
        "Shanghai corpus pin differs from reviewed acceptance contract"
    );
    let members = load_shanghai_inventory(root)?;
    let output = resolve_contained_path(root, Path::new("local-tests/b2-corpus"))?;
    std::fs::create_dir_all(&output)?;
    let archive = fetch_shanghai_archive(root, &output, artifacts, report)?;
    let destination = resolve_contained_path(&output, Path::new("selected-shanghai"))?;
    if !destination.exists() {
        let names = read_archive_listing(&archive, false)?;
        validate_archive_members(&names, "fixtures")?;
        validate_archive_links(&read_archive_listing(&archive, true)?, "fixtures")?;
        let available: std::collections::BTreeSet<_> = names.lines().collect();
        ensure!(
            members
                .keys()
                .all(|member| available.contains(member.as_str())),
            "pinned archive lacks required Shanghai members"
        );
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let staged =
            resolve_contained_path(&output, Path::new(&format!("selected-stage-{nonce}")))?;
        std::fs::create_dir(&staged)?;
        let archive_arg = archive
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("UTF-8 archive path"))?;
        let staged_arg = staged
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("UTF-8 staging path"))?;
        let mut arguments = vec![
            "--extract",
            "--gzip",
            "--no-same-owner",
            "--no-same-permissions",
            "--keep-old-files",
            "--file",
            archive_arg,
            "--directory",
            staged_arg,
            "--",
        ];
        arguments.extend(members.keys().map(String::as_str));
        require_command_success(root, artifacts, report, "tar", &arguments)?;
        validate_shanghai_extraction(&staged, &members)?;
        std::fs::rename(staged, &destination)?;
    }
    validate_shanghai_extraction(&destination, &members)?;
    report.tool_environment.insert(
        "EVE_SHANGHAI_FIXTURES".into(),
        destination
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("UTF-8 corpus path"))?
            .into(),
    );
    Ok(destination)
}

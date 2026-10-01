// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{ARCHIVE_SHA256, SOURCE_REVISION, ShanghaiInventory};
use anyhow::{Result, ensure};
use std::{
    collections::BTreeMap,
    path::{Component, Path},
};

/// Load all pinned Shanghai families; no caller-selected partial corpus is allowed.
pub(super) fn load_shanghai_inventory(root: &Path) -> Result<BTreeMap<String, String>> {
    let directory = root.join("tests/acceptance/serial-rpc/fixtures/corpus/shanghai");
    let mut members = BTreeMap::new();
    for family in [
        "berlin",
        "byzantium",
        "cancun",
        "constantinople",
        "frontier",
        "homestead",
        "istanbul",
        "london",
        "osaka",
        "ported_static",
        "shanghai",
        "transaction_validation",
    ] {
        let path = directory.join(format!("{family}.json"));
        let metadata = std::fs::symlink_metadata(&path)?;
        ensure!(
            metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() <= 65_536,
            "invalid corpus inventory file"
        );
        let manifest: ShanghaiInventory = serde_json::from_slice(&std::fs::read(path)?)?;
        ensure!(
            manifest.schema_version == 1
                && manifest.source_revision == SOURCE_REVISION
                && manifest.archive_sha256 == ARCHIVE_SHA256
                && manifest.target_fork == "Shanghai"
                && manifest.family == family,
            "Shanghai inventory identity mismatch"
        );
        for (path, digest) in manifest.files {
            ensure!(
                path.len() <= 512
                    && !path.contains('\\')
                    && !path.bytes().any(|byte| byte.is_ascii_control())
                    && Path::new(&path)
                        .components()
                        .all(|part| matches!(part, Component::Normal(_)))
                    && path.ends_with(".json")
                    && (path.starts_with("fixtures/state_tests/for_shanghai/")
                        || path.starts_with("fixtures/transaction_tests/for_shanghai/")),
                "invalid Shanghai member path"
            );
            ensure!(
                digest.len() == 64
                    && digest
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
                "invalid member digest"
            );
            ensure!(
                members.insert(path, digest).is_none(),
                "duplicate Shanghai inventory member"
            );
        }
    }
    ensure!(
        members.len() == 195
            && members
                .keys()
                .filter(|path| path.starts_with("fixtures/state_tests/"))
                .count()
                == 194
            && members
                .keys()
                .filter(|path| path.starts_with("fixtures/transaction_tests/"))
                .count()
                == 1,
        "incomplete Shanghai file inventory"
    );
    Ok(members)
}

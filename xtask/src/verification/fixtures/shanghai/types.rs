// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ShanghaiInventory {
    pub schema_version: u32,
    pub source_revision: String,
    pub archive_sha256: String,
    pub target_fork: String,
    pub family: String,
    pub files: BTreeMap<String, String>,
}

pub(super) const ARCHIVE_BYTES: u64 = 540_487_005;
pub(super) const ARCHIVE_SHA256: &str =
    "1280540950a4c3470a421416b6f35458a9b635827265c29e5aef1ae839ae1788";
pub(super) const SOURCE_REVISION: &str = "abbe05777ab83fb94ce18c425daaa7ab79e779c1";
pub(super) const EXTRACTED_BYTES: u64 = 45_005_468;
pub(super) const ARCHIVE_URL: &str =
    "https://github.com/ethereum/execution-specs/releases/download/tests%40v20.0.2/fixtures.tar.gz";

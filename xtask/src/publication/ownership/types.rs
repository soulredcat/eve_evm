// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub(super) const REDCAT_LICENSE: &str = "LicenseRef-Redcat-Permission-Only";
pub(super) const COPYRIGHT: &str = "SPDX-FileCopyrightText: 2026 Redcat";
pub(super) const LICENSE_NOTICE: &str =
    "SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only";
pub(super) const PERMISSION: &str = "Use requires prior written permission from Redcat.";

#[derive(Debug, Default, Serialize)]
pub struct OwnershipReport {
    pub policy_version: u32,
    pub coverage: OwnershipCoverage,
    pub files: Vec<OwnershipFile>,
    pub violations: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct OwnershipCoverage {
    pub inline_first_party: usize,
    pub annotated_first_party: usize,
    pub upstream: usize,
    pub license_text: usize,
}

#[derive(Debug, Serialize)]
pub struct OwnershipFile {
    pub path: String,
    pub kind: String,
    pub violations: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ReusePolicy {
    pub version: u32,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Annotation {
    pub path: StringValues,
    pub precedence: Option<String>,
    #[serde(rename = "SPDX-FileCopyrightText")]
    pub copyright: StringValues,
    #[serde(rename = "SPDX-License-Identifier")]
    pub license: String,
    #[serde(rename = "SPDX-FileComment")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(super) enum StringValues {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Default)]
pub(super) struct AnnotationInventory {
    pub annotations: BTreeMap<String, Annotation>,
    pub violations: Vec<String>,
}

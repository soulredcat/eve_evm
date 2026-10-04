// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructurePolicy {
    pub version: u32,
    pub current_bulk: u8,
    #[serde(default)]
    pub exclusions: Vec<Exclusion>,
    #[serde(default)]
    pub size_reviews: Vec<SizeReview>,
    #[serde(default)]
    pub exceptions: Vec<SizeException>,
    #[serde(default)]
    pub adapters: Vec<AdapterReview>,
    #[serde(default)]
    pub adapter_files: Vec<String>,
    #[serde(default)]
    pub generated_modules: Vec<GeneratedModuleReview>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exclusion {
    pub path: String,
    pub kind: String,
    pub source: String,
    pub reason: String,
    pub generator: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SizeReview {
    pub path: String,
    pub lines: usize,
    pub reason: String,
    pub reviewer: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SizeException {
    pub path: String,
    pub lines: usize,
    pub reason: String,
    pub reviewer: String,
    pub split_task: String,
    pub expires_bulk: u8,
    #[serde(default)]
    pub related_tests: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterReview {
    pub path: String,
    pub external_trait: String,
    pub reason: String,
    pub reviewer: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedModuleReview {
    pub path: String,
    pub generator: String,
    pub source_manifest: String,
    pub source_sha256: String,
    pub generator_sha256: String,
    pub manifest_sha256: String,
    pub reason: String,
    pub reviewer: String,
}

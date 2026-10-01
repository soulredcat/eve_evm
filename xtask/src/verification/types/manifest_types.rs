// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateRegistry {
    pub version: u32,
    pub gates: Vec<GateRegistration>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateRegistration {
    pub id: String,
    pub manifest: String,
    pub implemented: bool,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GateManifest {
    pub version: u32,
    pub id: String,
    pub profile: String,
    pub groups: Vec<String>,
    pub required_inputs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestGroup {
    pub version: u32,
    pub package: String,
    pub requirements: Vec<String>,
    pub tests: Vec<String>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub doc_tests: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementManifest {
    pub version: u32,
    pub cases: Vec<RequirementCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementCase {
    pub id: String,
    pub owner_gate: String,
    pub status: String,
    pub expected: String,
    pub fixture: String,
}

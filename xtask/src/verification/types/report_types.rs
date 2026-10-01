// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Default, Debug, Serialize)]
pub struct VerificationReport {
    pub schema_version: u32,
    pub started_unix_ms: u128,
    pub completed_unix_ms: u128,
    pub host_platform: String,
    pub compiler_identity: String,
    pub topology: String,
    pub genesis_identity: String,
    pub profile_activation: String,
    pub key_epochs: String,
    pub requested: Vec<String>,
    pub status: String,
    pub profile: String,
    pub revision: String,
    pub dirty: bool,
    pub source_sha256: String,
    pub inputs: BTreeMap<String, String>,
    pub tool_environment: BTreeMap<String, String>,
    pub tool_evidence: Option<serde_json::Value>,
    pub commands: Vec<CommandEvidence>,
    pub groups: Vec<GroupEvidence>,
    pub test_count: usize,
    pub errors: Vec<String>,
    pub pending: Vec<String>,
    pub registered_requirements: BTreeMap<String, String>,
    pub core_requirement_status: BTreeMap<String, String>,
    pub artifact_directory: String,
}

#[derive(Debug, Serialize)]
pub struct CommandEvidence {
    pub program: String,
    pub arguments: Vec<String>,
    pub exit_code: Option<i32>,
    pub spawn_error: Option<String>,
    pub elapsed_ms: u128,
    pub stdout: String,
    pub stderr: String,
    pub stdout_sha256: String,
    pub stderr_sha256: String,
}

#[derive(Debug, Serialize)]
pub struct GroupEvidence {
    pub package: String,
    pub requirements: Vec<String>,
    pub discovered: usize,
    pub passed: usize,
    pub ignored: usize,
}

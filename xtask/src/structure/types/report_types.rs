// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct StructureReport {
    pub policy_version: u32,
    pub current_bulk: u8,
    pub files: Vec<FileReport>,
    pub exclusions: Vec<String>,
    pub warnings: Vec<String>,
    pub violations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FileReport {
    pub path: String,
    pub kind: String,
    pub physical_lines: usize,
    /// AST coverage is distinct from physical/path checks and bulk compiler tests.
    pub syntax_coverage: String,
    pub operations: Vec<String>,
    pub violations: Vec<String>,
}

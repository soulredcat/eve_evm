// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::path::PathBuf;

#[derive(Default)]
pub struct SourceEdgeInventory {
    pub targets: Vec<SourceTarget>,
    pub violations: Vec<String>,
}

pub struct SourceTarget {
    pub path: PathBuf,
    pub test_only: bool,
}

#[derive(Default)]
pub struct ModulePathAttributes {
    pub paths: Vec<String>,
    pub has_unconditional_path: bool,
}

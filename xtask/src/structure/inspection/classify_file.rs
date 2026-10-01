// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::types::policy_types::StructurePolicy;
use std::path::Path;

pub fn classify_file(path: &str, policy: &StructurePolicy, operation_count: usize) -> &'static str {
    let file = Path::new(path);
    let name = file
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    match file.extension().and_then(|extension| extension.to_str()) {
        Some("rs") => {
            if path.starts_with("tests/") || path.contains("/tests/") {
                "test"
            } else if policy.adapters.iter().any(|adapter| adapter.path == path) {
                "adapter"
            } else if matches!(name, "mod.rs" | "lib.rs") {
                "facade"
            } else if matches!(name, "main.rs" | "build.rs") {
                "entry"
            } else if operation_count == 0 {
                "declaration"
            } else {
                "behavior"
            }
        }
        Some("md" | "toml" | "json" | "yml" | "yaml" | "txt" | "lock") => "document",
        Some("ts" | "sol") if super::is_external_test_fixture::is_external_test_fixture(path) => {
            "test"
        }
        None if matches!(
            name,
            ".gitignore" | ".gitattributes" | "NOTICE" | "LICENSE" | "SHA256SUMS"
        ) =>
        {
            "document"
        }
        _ => "unsupported",
    }
}

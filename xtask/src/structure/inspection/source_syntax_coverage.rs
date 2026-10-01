// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::path::Path;

/// Report exactly what the structure scan parses; external fixture syntax is
/// verified by its mandatory compiler gate, not inferred from an empty inventory.
pub fn source_syntax_coverage(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("rs") => "RUST_AST",
        Some("ts" | "sol") if super::is_external_test_fixture::is_external_test_fixture(path) => {
            "TEST_SOURCE_REQUIRES_BULK_COMPILER"
        }
        Some("ts" | "sol") => "NOT_IMPLEMENTED",
        _ => "NOT_APPLICABLE",
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// External fixture languages are permitted only after a test boundary, never
/// by adding a tests directory beneath a production source boundary.
pub fn is_external_test_fixture(path: &str) -> bool {
    if path.starts_with("tests/") {
        return true;
    }
    let Some(test_boundary) = path.find("/tests/") else {
        return false;
    };
    path.find("/src/")
        .is_none_or(|source_boundary| test_boundary < source_boundary)
}

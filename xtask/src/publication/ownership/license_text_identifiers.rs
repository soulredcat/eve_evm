// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Expand only the reviewed ZIP215 conjunction; this is not a general SPDX expression parser.
pub(super) fn license_text_identifiers(expression: &str) -> Option<Vec<&str>> {
    if expression == "Apache-2.0 AND BSD-3-Clause" {
        Some(vec!["Apache-2.0", "BSD-3-Clause"])
    } else if !expression.is_empty()
        && expression
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.'))
    {
        Some(vec![expression])
    } else {
        None
    }
}

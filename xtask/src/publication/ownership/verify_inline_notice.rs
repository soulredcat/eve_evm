// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    notice_markers::notice_markers,
    types::{COPYRIGHT, LICENSE_NOTICE, PERMISSION},
};
use std::path::Path;

pub(super) fn verify_inline_notice(path: &str, source: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let mut source = source.strip_prefix('\u{feff}').unwrap_or(source);
    if source.starts_with("#!") && !source.starts_with("#![") {
        source = source.split_once('\n').map_or("", |(_, body)| body);
    }
    let extension = Path::new(path).extension().and_then(|value| value.to_str());
    let expected: Vec<String> = match extension {
        Some("rs" | "ts" | "sol") => [COPYRIGHT, LICENSE_NOTICE, PERMISSION]
            .map(|value| format!("// {value}"))
            .into(),
        Some("toml" | "yml" | "yaml") => [COPYRIGHT, LICENSE_NOTICE, PERMISSION]
            .map(|value| format!("# {value}"))
            .into(),
        Some("md") => [COPYRIGHT, LICENSE_NOTICE, PERMISSION]
            .map(|value| format!("<!-- {value} -->"))
            .into(),
        _ if matches!(path, ".gitignore" | ".gitattributes") => {
            [COPYRIGHT, LICENSE_NOTICE, PERMISSION]
                .map(|value| format!("# {value}"))
                .into()
        }
        _ => {
            violations.push(format!(
                "{path}: unannotated noncommentable or unsupported first-party format"
            ));
            return violations;
        }
    };
    let lines: Vec<_> = source.lines().take(4).collect();
    if lines.len() != 4
        || !lines
            .iter()
            .take(3)
            .zip(&expected)
            .all(|(actual, expected)| actual == expected)
        || !lines[3].is_empty()
    {
        violations.push(format!(
            "{path}: missing exact three-line Redcat notice and blank separator"
        ));
    }
    if notice_markers(source) != [COPYRIGHT.to_owned(), LICENSE_NOTICE.to_owned()] {
        violations.push(format!(
            "{path}: missing, duplicate or contradictory ownership declarations"
        ));
    }
    violations
}

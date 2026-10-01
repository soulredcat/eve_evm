// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::path::Path;

pub fn has_forbidden_source_segment(path: &str, production: bool) -> bool {
    let file = Path::new(path);
    let stem = file
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let segments = if production {
        path.split('/')
            .chain(std::iter::once(stem))
            .collect::<Vec<_>>()
    } else {
        vec![stem]
    };
    segments.iter().any(|segment| {
        matches!(
            *segment,
            "utils" | "helpers" | "common" | "shared" | "misc" | "manager"
        ) || segment.strip_prefix("part").is_some_and(|suffix| {
            !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
        })
    })
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Inspect comment declarations, avoiding quoted examples and output strings.
pub(super) fn notice_markers(source: &str) -> Vec<String> {
    let mut ignored = false;
    let mut markers = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if line.contains("REUSE-IgnoreStart") && line.starts_with("<!--") {
            ignored = true;
            continue;
        }
        if line.contains("REUSE-IgnoreEnd") && line.starts_with("<!--") {
            ignored = false;
            continue;
        }
        if ignored {
            continue;
        }
        let content = ["<!--", "///", "//!", "//", "/*", "*", "#"]
            .iter()
            .find_map(|prefix| line.strip_prefix(prefix));
        if let Some(content) = content {
            let content = content
                .trim()
                .trim_end_matches("-->")
                .trim_end_matches("*/")
                .trim();
            if content.starts_with("SPDX-License-Identifier:")
                || content.starts_with("SPDX-FileCopyrightText:")
            {
                markers.push(content.to_owned());
            }
        }
    }
    markers
}

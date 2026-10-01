// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    notice_markers::notice_markers,
    types::{Annotation, REDCAT_LICENSE},
    upstream_license::upstream_license,
    value_items::value_items,
};

pub(super) fn validate_annotation(
    path: &str,
    annotation: &Annotation,
    source: &str,
) -> Vec<String> {
    let mut violations = Vec::new();
    let copyright = value_items(&annotation.copyright);
    if copyright.is_empty() || copyright.iter().any(|value| value.trim().is_empty()) {
        violations.push(format!("{path}: missing annotation copyright information"));
    }
    if let Some(expected) = upstream_license(path) {
        if annotation.license != expected
            || copyright
                .iter()
                .any(|value| value.to_ascii_lowercase().contains("redcat"))
            || notice_markers(source)
                .iter()
                .any(|value| value.to_ascii_lowercase().contains("redcat"))
            || annotation.comment.as_deref().is_some_and(|value| {
                value
                    .to_ascii_lowercase()
                    .contains("permission from redcat")
            })
        {
            violations.push(format!(
                "{path}: Redcat ownership or changed license on known upstream material"
            ));
        }
        if notice_markers(source)
            .iter()
            .filter_map(|marker| marker.strip_prefix("SPDX-License-Identifier:"))
            .any(|license| license.trim() != annotation.license)
        {
            violations.push(format!(
                "{path}: annotation contradicts preserved upstream inline license"
            ));
        }
    } else {
        if annotation.license != REDCAT_LICENSE
            || !copyright
                .iter()
                .all(|value| value == "2026 Redcat" || value.starts_with("2026 Redcat ("))
            || !annotation
                .comment
                .as_deref()
                .is_some_and(|value| value.contains("prior written permission from Redcat"))
        {
            violations.push(format!(
                "{path}: incomplete or contradictory first-party annotation"
            ));
        }
        if !notice_markers(source).is_empty() {
            violations.push(format!(
                "{path}: external annotation cannot hide inline ownership declarations"
            ));
        }
        if !path.ends_with(".json")
            && path != "Cargo.lock"
            && path != "validator/components/consensus-comet/vendor/SHA256SUMS"
        {
            violations.push(format!(
                "{path}: commentable first-party file requires its inline notice"
            ));
        }
    }
    violations
}

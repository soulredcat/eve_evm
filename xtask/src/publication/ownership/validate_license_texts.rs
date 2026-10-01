// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    license_text_identifiers::license_text_identifiers,
    types::{AnnotationInventory, REDCAT_LICENSE},
    upstream_license::upstream_license,
};
use crate::structure::inspection::resolve_source_path::resolve_source_path;
use std::{collections::BTreeSet, path::Path};

pub(super) fn validate_license_texts(
    root: &Path,
    sources: &[String],
    inventory: &AnnotationInventory,
) -> Vec<String> {
    let mut violations = Vec::new();
    let mut expressions: BTreeSet<&str> = inventory
        .annotations
        .values()
        .map(|entry| entry.license.as_str())
        .collect();
    expressions.insert(REDCAT_LICENSE);
    let mut licenses = BTreeSet::new();
    for expression in expressions {
        match license_text_identifiers(expression) {
            Some(identifiers) => licenses.extend(identifiers),
            None => violations.push(format!(
                "Unsupported ownership license expression: {expression}"
            )),
        }
    }
    for license in &licenses {
        let path = format!("LICENSES/{license}.txt");
        if !sources.contains(&path)
            || resolve_source_path(root, &path)
                .ok()
                .and_then(|path| std::fs::read(path).ok())
                .is_none_or(|bytes| bytes.iter().all(u8::is_ascii_whitespace))
        {
            violations.push(format!("{path}: missing or empty license text"));
        }
    }
    for path in sources.iter().filter(|path| path.starts_with("LICENSES/")) {
        if !licenses
            .iter()
            .any(|license| path == &format!("LICENSES/{license}.txt"))
        {
            violations.push(format!("{path}: unexpected, nested or unused license text"));
        }
        if upstream_license(path).is_some()
            && resolve_source_path(root, path)
                .and_then(|path| Ok(std::fs::read_to_string(path)?))
                .is_ok_and(|text| {
                    text.contains("Copyright (c) 2026 Redcat")
                        || text.contains("LicenseRef-Redcat-Permission-Only")
                })
        {
            violations.push(format!(
                "{path}: Redcat notice added to preserved upstream license text"
            ));
        }
    }
    for path in ["LICENSE", "LICENSES/LicenseRef-Redcat-Permission-Only.txt"] {
        match resolve_source_path(root, path).and_then(|path| Ok(std::fs::read_to_string(path)?)) {
            Ok(text)
                if text.contains("Copyright (c) 2026 Redcat")
                    && text.contains(REDCAT_LICENSE)
                    && text
                        .to_ascii_lowercase()
                        .contains("prior written permission from redcat")
                    && text.contains("platform terms")
                    && text.contains("applicable law") => {}
            _ => violations.push(format!(
                "{path}: missing first-party permission terms or rights-preservation clauses"
            )),
        }
    }
    violations
}

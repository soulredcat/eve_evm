// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    types::{AnnotationInventory, OwnershipFile},
    upstream_license::upstream_license,
    validate_annotation::validate_annotation,
    verify_inline_notice::verify_inline_notice,
    verify_upstream_digest::verify_upstream_digest,
};
use crate::structure::{
    inspection::resolve_source_path::resolve_source_path, types::policy_types::StructurePolicy,
};
use anyhow::Result;
use std::path::Path;

pub(super) fn inspect_ownership_file(
    root: &Path,
    path: &str,
    inventory: &AnnotationInventory,
    policy: &StructurePolicy,
) -> Result<OwnershipFile> {
    let bytes = std::fs::read(resolve_source_path(root, path)?)?;
    let source = String::from_utf8_lossy(&bytes);
    let mut violations = verify_upstream_digest(root, path, policy);
    let kind;
    if path == "LICENSE" || path.starts_with("LICENSES/") {
        kind = "license_text";
        if std::str::from_utf8(&bytes).is_err() {
            violations.push(format!("{path}: license text must be UTF-8 plain text"));
        }
    } else if let Some(annotation) = inventory.annotations.get(path) {
        kind = if upstream_license(path).is_some() {
            "upstream"
        } else {
            "annotated_first_party"
        };
        violations.extend(validate_annotation(path, annotation, &source));
    } else if upstream_license(path).is_some() {
        kind = "upstream";
        violations.push(format!(
            "{path}: known upstream file lacks its exact attribution annotation"
        ));
    } else {
        kind = "inline_first_party";
        match std::str::from_utf8(&bytes) {
            Ok(source) => violations.extend(verify_inline_notice(path, source)),
            Err(_) => violations.push(format!(
                "{path}: non-UTF-8 first-party file needs reviewed external metadata"
            )),
        }
    }
    Ok(OwnershipFile {
        path: path.to_owned(),
        kind: kind.into(),
        violations,
    })
}

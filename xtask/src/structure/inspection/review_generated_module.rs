// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::{
    inspection::compute_source_digest::compute_source_digest,
    types::{policy_types::GeneratedModuleReview, syntax_types::SyntaxInventory},
};
use std::path::Path;

pub fn review_generated_module(
    root: &Path,
    review: &GeneratedModuleReview,
    inventory: &SyntaxInventory,
) -> bool {
    if !inventory.operations.is_empty()
        || inventory.executable_initializers > 0
        || inventory.complex_closures > 0
        || inventory.includes == 0
        || inventory.includes != inventory.opaque_macros
        || !review.generator.ends_with(".rs")
        || review.reason.trim().is_empty()
        || review.reviewer.trim().is_empty()
    {
        return false;
    }
    [
        (&review.path, &review.source_sha256),
        (&review.generator, &review.generator_sha256),
        (&review.source_manifest, &review.manifest_sha256),
    ]
    .iter()
    .all(|(path, expected)| {
        compute_source_digest(root, path).is_ok_and(|actual| actual.eq_ignore_ascii_case(expected))
    })
}

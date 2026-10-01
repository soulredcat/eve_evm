// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Exact repository ownership policy; not complete REUSE or SPDX certification.
pub mod check_ownership;
mod inspect_ownership_file;
mod load_annotations;
mod notice_markers;
pub mod types;
mod upstream_license;
mod validate_annotation;
mod validate_license_texts;
mod value_items;
mod verify_inline_notice;
mod verify_upstream_digest;

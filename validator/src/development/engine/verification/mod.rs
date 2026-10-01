// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod engine_file_identity;
mod engine_process_image_identity;
mod hash_engine_file;
mod hash_engine_image_file;
mod read_engine_file;
mod types;
mod validate_engine_image_binding;
mod validate_engine_namespace;
mod verify_engine_binary;
#[cfg(test)]
mod verify_test_engine_image;
pub(super) use engine_file_identity::engine_file_identity;
pub(crate) use engine_process_image_identity::engine_process_image_identity;
pub(super) use hash_engine_file::hash_engine_file;
pub(super) use hash_engine_image_file::hash_engine_image_file;
pub(super) use read_engine_file::read_engine_file;
pub(crate) use types::{EngineImageIdentity, VerifiedEngineImage};
pub(crate) use validate_engine_image_binding::validate_engine_image_binding;
pub(super) use validate_engine_namespace::validate_engine_namespace;
pub(super) use verify_engine_binary::verify_engine_binary;
#[cfg(test)]
pub(crate) use verify_test_engine_image::verify_test_engine_image;

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod initialize_opaque_namespace;
mod open_opaque_database;
mod open_opaque_record_repository;
mod sync_opaque_directory;
mod validate_opaque_namespace;
pub(super) use initialize_opaque_namespace::initialize_opaque_namespace;
pub(super) use open_opaque_database::open_opaque_database;
pub use open_opaque_record_repository::open_opaque_record_repository;
pub(super) use sync_opaque_directory::sync_opaque_directory;
pub(super) use validate_opaque_namespace::validate_opaque_namespace;

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod acquire_engine_lease;
mod engine_artifact_prefix;
mod engine_lease_drop_adapter;
mod finish_engine_lease;
mod initialize_engine_home;
mod read_engine_home_marker;
mod release_engine_lease;
mod sync_engine_directory;
mod sync_native_engine_files;
mod types;
mod validate_engine_home;
mod write_owned_engine_file;
pub(super) use acquire_engine_lease::acquire_engine_lease;
pub(super) use engine_artifact_prefix::engine_artifact_prefix;
pub(crate) use initialize_engine_home::initialize_engine_home;
pub(super) use read_engine_home_marker::read_engine_home_marker;
pub(super) use release_engine_lease::release_engine_lease;
pub(super) use sync_engine_directory::sync_engine_directory;
pub(super) use sync_native_engine_files::sync_native_engine_files;
pub(super) use types::EngineLease;
pub(super) use validate_engine_home::validate_engine_home;
pub(super) use write_owned_engine_file::write_owned_engine_file;

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests;

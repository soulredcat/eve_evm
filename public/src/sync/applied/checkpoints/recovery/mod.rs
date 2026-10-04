// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Strict authenticated existing-base recovery before any public state publication or worker.
mod create_checkpoint_genesis_publication;
mod find_latest_checkpoint_base;
mod open_checkpoint_recovery_root;
mod open_segmented_applied_state_service_with_checkpoints;
mod read_discovered_checkpoint_base;
mod recover_checkpoint_applied_prefix;
mod recover_checkpoint_suffix;
mod reopen_checkpoint_artifacts;
mod start_recovered_segmented_worker;
mod types;
mod validate_reopened_checkpoint_base;
pub use open_segmented_applied_state_service_with_checkpoints::open_segmented_applied_state_service_with_checkpoints;
pub(in crate::sync::applied) use recover_checkpoint_applied_prefix::recover_checkpoint_applied_prefix;
pub(in crate::sync::applied) use start_recovered_segmented_worker::start_recovered_segmented_worker;

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::MasterSyncConfig;
use crate::development::config::development_harness_budget::development_harness_budget;
use eve_finality_verifier::MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES;
use std::path::PathBuf;

/// Explicit bounded local archive profile, not a full-core capacity or retention-window claim.
pub fn master_sync_development_config(root: PathBuf, data: PathBuf) -> MasterSyncConfig {
    MasterSyncConfig {
        root,
        data,
        mode: "MASTER_SYNC_ONLY".into(),
        acknowledge_unsafe_development: false,
        storage: development_harness_budget(),
        working_bytes: 512 * 1_048_576,
        maximum_proof_bytes: MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES,
        maximum_proof_files: 4_096,
        maximum_archive_bytes: 1_073_741_824,
        maximum_retained_commit_bytes: 1_073_741_824,
    }
}

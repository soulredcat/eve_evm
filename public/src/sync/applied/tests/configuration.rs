// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{config, empty_chain};
use crate::sync::applied::{AppliedError, open_applied_state_service};

#[test]
fn valid_large_queue_profile_rejects_metadata_capacity_before_namespace_creation() {
    let directory = tempfile::tempdir().unwrap();
    let chain = empty_chain();
    let path = directory.path().join("metadata-limit");
    let mut configured = config(&path, &chain);
    // One-byte payloads make the declared byte pools small while queue slot count
    // remains large. This tests startup accounting, not usable recovery framing.
    configured.public_budget.maximum_record_bytes = 1;
    configured.public_budget.maximum_batch_bytes = 1;
    configured.public_budget.queue_batches = 1_000_000;
    configured.public_budget.queue_bytes = 1_000_000;
    configured.maximum_recovery_payload_bytes = 1;
    eve_node_policy::validate_public_budget(configured.public_budget).unwrap();
    eve_storage::records::validate_opaque_record_budget(&configured.repository_budget).unwrap();
    assert!(matches!(
        open_applied_state_service(configured, &chain.genesis),
        Err(AppliedError::EstimatedCapacity)
    ));
    assert!(!path.exists());
}

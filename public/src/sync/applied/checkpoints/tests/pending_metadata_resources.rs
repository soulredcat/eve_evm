// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*, fixtures::limits, pending_metadata_fixture::*, reopen_fixtures::private_archive,
};
use crate::sync::applied::{
    AppliedError, finish_applied_state_service, observe_estimated_working, reserve_applied_working,
};
use std::fs::File;

#[test]
fn exhausted_actual_owner_pool_refuses_both_metadata_paths_before_io_or_namespace_creation() {
    let fixture = fixture();
    let observed = observe_estimated_working(&fixture.reader).unwrap();
    let remaining = usize::try_from(observed.limit - observed.reserved_estimated_bytes).unwrap();
    let held = reserve_applied_working(&fixture.reader, remaining).unwrap();
    let staging = private_archive();
    let root = File::open(staging.path()).unwrap();
    assert!(matches!(
        repair_applied_checkpoint_content_pending(&fixture.owner, &root, &[], &[], limits()),
        Err(CheckpointAppliedError::Applied(
            AppliedError::EstimatedCapacity
        ))
    ));
    assert!(matches!(
        repair_applied_checkpoint_proof_pending(&fixture.artifacts.content, &root, &[]),
        Err(CheckpointAppliedError::Applied(
            AppliedError::EstimatedCapacity
        ))
    ));
    assert_eq!(std::fs::read_dir(staging.path()).unwrap().count(), 0);
    drop(held);
    assert_eq!(
        observe_estimated_working(&fixture.reader)
            .unwrap()
            .reserved_estimated_bytes,
        observed.reserved_estimated_bytes
    );
    drop(fixture.artifacts);
    drop(finish_applied_state_service(fixture.owner));
}

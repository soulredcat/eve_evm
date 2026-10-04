// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::*, fixtures::limits};
use crate::sync::applied::{
    AppliedError, applied_owner_reader, finish_applied_state_service, observe_estimated_working,
    open_segmented_applied_state_service, reserve_applied_working,
    segmented::tests::fixtures::{configuration, logical_chain},
};
use std::fs::File;

#[test]
fn exhausted_actual_owner_pool_rejects_before_allocating_a_manifest_namespace() {
    let directory = tempfile::tempdir().unwrap();
    let archive = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (owner, _) = open_segmented_applied_state_service(
        configuration(&directory.path().join("store"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let reader = applied_owner_reader(&owner);
    let observed = observe_estimated_working(&reader).unwrap();
    let remaining = usize::try_from(observed.limit - observed.reserved_estimated_bytes).unwrap();
    let held = reserve_applied_working(&reader, remaining).unwrap();
    assert!(matches!(
        begin_applied_checkpoint_transfer(
            &owner,
            &File::open(archive.path()).unwrap(),
            &[],
            &[],
            limits()
        ),
        Err(CheckpointAppliedError::Applied(
            AppliedError::EstimatedCapacity
        ))
    ));
    assert_eq!(std::fs::read_dir(archive.path()).unwrap().count(), 0);
    drop(held);
    drop(finish_applied_state_service(owner));
}

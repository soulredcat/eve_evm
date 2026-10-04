// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    reopen_fixtures::{completed_checkpoint, recovery_config, reopen},
};
use crate::sync::applied::{
    finish_applied_state_service, segmented::tests::fixtures::configuration,
};
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, read_opaque_record,
};

#[test]
fn a_malformed_newer_typed_base_is_not_ignored_in_favor_of_an_older_valid_base() {
    let fixture = completed_checkpoint(false);
    let (owner, _) = reopen(&fixture).unwrap();
    let shutdown = finish_applied_state_service(owner);
    let mut repository = shutdown.repository.unwrap();
    let old = read_opaque_record(&repository, 1).unwrap().unwrap();
    let mut corrupted = old.payload.clone();
    *corrupted.last_mut().unwrap() ^= 1;
    let head = opaque_record_cursor(&repository).unwrap();
    compare_and_append_opaque_records(&mut repository, head, &[corrupted]).unwrap();
    assert_eq!(read_opaque_record(&repository, 1).unwrap().unwrap(), old);
    drop(repository);
    assert!(matches!(
        reopen(&fixture),
        Err(CheckpointAppliedError::Base(_))
    ));
}

#[test]
fn a_foreign_opaque_namespace_is_refused_without_changing_the_original_store() {
    let fixture = completed_checkpoint(false);
    let mut config = configuration(&fixture.database.path().join("store"), &fixture.chain);
    config.application.identity.owner[0] ^= 1;
    assert!(
        open_segmented_applied_state_service_with_checkpoints(
            config,
            recovery_config(fixture.archive.path()),
            &fixture.chain.genesis
        )
        .is_err()
    );
    let (owner, _) = reopen(&fixture).unwrap();
    drop(finish_applied_state_service(owner));
}

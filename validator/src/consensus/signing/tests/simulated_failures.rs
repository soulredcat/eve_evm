// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{temporary_fixture, test_key, vote_request};
use crate::consensus::signing::{
    fence_signer, open_durable_signer, sign_vote, signer_status, types::SimulatedSignerFailure,
};
use eve_storage::records::development_opaque_record_budget;

#[test]
fn simulated_before_append_and_after_sync_fence_without_releasing_signature() {
    for failure in [
        SimulatedSignerFailure::BeforeWrite,
        SimulatedSignerFailure::AfterSync,
    ] {
        let (_directory, fixture) = temporary_fixture();
        let path = fixture.root.join("signing");
        let mut signer = open_durable_signer(
            &path,
            fixture.config.clone(),
            test_key(1),
            fixture.service.clone(),
            development_opaque_record_budget(),
        )
        .unwrap();
        signer.simulated_failure = Some(failure);
        let request = vote_request(&fixture.config);
        assert!(sign_vote(&mut signer, request.clone(), None).is_err());
        assert!(signer_status(&signer).fenced);
        assert!(sign_vote(&mut signer, request.clone(), None).is_err());
        drop(signer);
        let mut reopened = open_durable_signer(
            &path,
            fixture.config,
            test_key(1),
            fixture.service,
            development_opaque_record_budget(),
        )
        .unwrap();
        assert_eq!(
            signer_status(&reopened).cursor.sequence,
            u64::from(failure == SimulatedSignerFailure::AfterSync)
        );
        sign_vote(&mut reopened, request, None).unwrap();
        assert_eq!(signer_status(&reopened).cursor.sequence, 1);
    }
}

#[test]
fn application_uncertainty_fences_even_an_already_durable_retry() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let vote = sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
    fence_signer(&mut signer);
    assert!(sign_vote(&mut signer, vote, None).is_err());
    assert!(signer_status(&signer).fenced);
    assert_eq!(signer_status(&signer).cursor.sequence, 1);
}

#[test]
fn actual_history_capacity_failure_fences_new_signing_without_pruning() {
    let (_directory, fixture) = temporary_fixture();
    let mut budget = development_opaque_record_budget();
    budget.maximum_retained_records = 1;
    let path = fixture.root.join("signing");
    let mut signer = open_durable_signer(
        &path,
        fixture.config.clone(),
        test_key(1),
        fixture.service.clone(),
        budget,
    )
    .unwrap();
    let first = sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
    let mut next = first.clone();
    next.r#type = 2;
    next.signature.clear();
    assert!(sign_vote(&mut signer, next, None).is_err());
    assert!(signer_status(&signer).fenced);
    drop(signer);
    let mut reopened = open_durable_signer(
        &path,
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        budget,
    )
    .unwrap();
    assert_eq!(
        sign_vote(&mut reopened, vote_request(&fixture.config), None).unwrap(),
        first
    );
}

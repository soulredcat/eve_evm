// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::{
    open_durable_signer, sign_vote, signer_status,
    tests::{temporary_fixture, test_key, vote_request},
};
use eve_storage::records::development_opaque_record_budget;

#[test]
fn earlier_hrs_refusal_becomes_native_remote_signer_error_without_signing() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let mut prior = vote_request(&fixture.config);
    prior.round = 2;
    sign_vote(&mut signer, prior, None).unwrap();
    let before = signer_status(&signer).cursor;
    let mut earlier = vote_request(&fixture.config);
    earlier.round = 1;
    let error = sign_vote(&mut signer, earlier, None).unwrap_err();
    let response =
        super::super::refuse_signing_request::refuse_signing_request(&signer, &before, error)
            .unwrap();
    assert_eq!(response.code, 1);
    assert_eq!(response.description, "signer height/round/step regression");
    assert_eq!(signer_status(&signer).cursor, before);
    assert!(!signer_status(&signer).fenced);
}

#[test]
fn unclassified_signing_error_remains_fatal() {
    let (_directory, fixture) = temporary_fixture();
    let signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let before = signer_status(&signer).cursor;
    let error = anyhow::anyhow!("durable signer write failed");
    let fatal =
        super::super::refuse_signing_request::refuse_signing_request(&signer, &before, error)
            .unwrap_err();
    assert_eq!(fatal.to_string(), "durable signer write failed");
}

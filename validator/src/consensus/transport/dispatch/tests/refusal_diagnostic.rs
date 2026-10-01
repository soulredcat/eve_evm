// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    approval::create_approval_registry,
    signing::{
        open_durable_signer, sign_vote, signer_status,
        tests::{temporary_fixture, test_key, vote_request},
    },
};
use eve_consensus_comet::wire::tendermint::types::{BlockId, PartSetHeader};
use eve_state::development_state_budget;
use eve_storage::records::development_opaque_record_budget;

#[test]
fn earlier_hrs_refusal_is_preserved_when_nonnil_recovery_cache_is_missing() {
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
    let signed = sign_vote(&mut signer, prior, None).unwrap();
    assert_eq!(signed.signature.len(), 64);
    let before = signer_status(&signer).cursor;
    let mut earlier = vote_request(&fixture.config);
    earlier.round = 1;
    earlier.block_id = Some(BlockId {
        hash: vec![9; 32],
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![8; 32],
        }),
    });
    let registry = create_approval_registry();
    let error = super::super::handle_signer_vote_request::handle_signer_vote_request(
        &mut signer,
        &registry,
        earlier,
        &development_state_budget(),
        64 * 1_048_576,
    )
    .unwrap_err();
    let causes = error.chain().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(causes, vec!["signer height/round/step regression"]);
    assert_eq!(signer_status(&signer).cursor, before);
    assert!(!signer_status(&signer).fenced);
}

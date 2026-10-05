// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{CHAIN, GENESIS_APP, Roster, accept, reject_unchanged};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    history::{initialize_genesis_history, initialize_trusted_header_history},
};

#[test]
fn local_genesis_roster_and_application_anchor_bind_the_first_native_header() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    assert_eq!(history.chain_id(), CHAIN);
    assert_eq!(history.height(), 0);
    assert!(history.block_id().is_none());
    let mut forged = roster.frame(1, None, &roster);
    forged.header.app_hash[0] ^= 1;
    roster.resign(&mut forged);
    reject_unchanged(&mut history, &forged, &roster);
    let mut foreign = roster.frame(1, None, &roster);
    foreign.header.chain_id = "another-native-network".into();
    roster.resign(&mut foreign);
    reject_unchanged(&mut history, &foreign, &roster);
    accept(&mut history, &roster.frame(1, None, &roster), &roster);
}

#[test]
fn locally_trusted_header_still_binds_its_full_block_identity_and_successor_set() {
    let roster = Roster::new(1, 4);
    let next = Roster::new(11, 4);
    let anchor = roster.frame(7, None, &next);
    let mut wrong_id = anchor.block_id.clone();
    wrong_id.hash[0] ^= 1;
    assert!(
        initialize_trusted_header_history(
            CHAIN,
            &anchor.header,
            &wrong_id,
            ConsensusAuthenticationRequirement::ClassicalDev,
        )
        .is_err()
    );
    let mut history = initialize_trusted_header_history(
        CHAIN,
        &anchor.header,
        &anchor.block_id,
        ConsensusAuthenticationRequirement::ClassicalDev,
    )
    .unwrap();
    let stale_set = roster.frame(8, Some(anchor.block_id.clone()), &roster);
    reject_unchanged(&mut history, &stale_set, &roster);
    accept(
        &mut history,
        &next.frame(8, Some(anchor.block_id), &next),
        &next,
    );
    // This test configures trust locally; no peer, freshness or EVE identity is proven.
}

#[test]
fn activated_hybrid_history_cannot_be_initialized_with_classical_authority() {
    let roster = Roster::new(1, 4);
    assert!(
        initialize_genesis_history(
            CHAIN,
            &roster.validators,
            GENESIS_APP,
            ConsensusAuthenticationRequirement::ClassicalAndMldsa65,
        )
        .is_err()
    );
    let anchor = roster.frame(1, None, &roster);
    assert!(
        initialize_trusted_header_history(
            CHAIN,
            &anchor.header,
            &anchor.block_id,
            ConsensusAuthenticationRequirement::ClassicalAndMldsa65,
        )
        .is_err()
    );
}

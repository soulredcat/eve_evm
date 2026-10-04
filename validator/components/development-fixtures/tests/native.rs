// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{HistoricalValidatorSet, verify_commit_certificate},
};
use eve_development_fixtures::{
    genesis::genesis,
    native::{frame, resign},
};

#[test]
fn seeded_native_certificates_are_real_signatures_bound_to_header_and_chain() {
    let genesis = genesis();
    let mut native = frame(&genesis, 1, None, [0x33; 32]);
    let set = HistoricalValidatorSet {
        height: 1,
        authentication: ConsensusAuthenticationRequirement::ClassicalDev,
        validators: native.validators.clone(),
    };
    let verified = verify_commit_certificate(
        &genesis.network_name,
        1,
        0,
        &native.id,
        &native.header,
        &native.commit,
        &set,
    )
    .unwrap();
    assert_eq!(
        (verified.signed_voting_power, verified.total_voting_power),
        (40_000, 40_000)
    );
    let original = native.commit.clone();
    native.header.app_hash[0] ^= 1;
    assert!(
        verify_commit_certificate(
            &genesis.network_name,
            1,
            0,
            &native.id,
            &native.header,
            &native.commit,
            &set
        )
        .is_err()
    );
    resign(&mut native);
    assert_ne!(native.commit, original);
    verify_commit_certificate(
        &genesis.network_name,
        1,
        0,
        &native.id,
        &native.header,
        &native.commit,
        &set,
    )
    .unwrap();
    native.commit.signatures[0].signature[0] ^= 1;
    assert!(
        verify_commit_certificate(
            &genesis.network_name,
            1,
            0,
            &native.id,
            &native.header,
            &native.commit,
            &set
        )
        .is_err()
    );
    resign(&mut native);
    assert!(
        verify_commit_certificate(
            "other-development-chain",
            1,
            0,
            &native.id,
            &native.header,
            &native.commit,
            &set
        )
        .is_err()
    );
}

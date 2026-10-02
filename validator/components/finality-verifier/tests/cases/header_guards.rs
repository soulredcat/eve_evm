// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{chain, genesis, native};
use eve_finality_verifier::{FinalityError, authenticate_current_application_version};
use eve_protocol_config::genesis::ScheduledUpgrade;
use eve_state::{
    B256, Bytes, SecurityProfile, development_state_budget, initialize_development_state,
};

#[test]
fn application_width_and_native_version_fail_before_advancing_eve_history() {
    let chain = chain::chain();
    let mut verifier = chain::verifier(&chain.genesis);
    for length in [0, 31, 33, 1024] {
        let mut malformed = native::frame(
            &chain.genesis,
            1,
            None,
            chain.commits[0].target.content_digest.0,
        );
        malformed.header.app_hash.resize(length, 0);
        native::resign(&mut malformed);
        assert_eq!(
            chain::reject(&mut verifier, &malformed),
            FinalityError::InvalidApplicationHashWidth
        );
    }
    let mut wrong_version = native::frame(
        &chain.genesis,
        1,
        None,
        chain.commits[0].target.content_digest.0,
    );
    wrong_version.header.version.as_mut().unwrap().app += 1;
    native::resign(&mut wrong_version);
    assert_eq!(
        chain::reject(&mut verifier, &wrong_version),
        FinalityError::WrongNativeApplicationVersion
    );
    chain::accept(&mut verifier, &chain.frames[0]);
    let mut wrong_next = native::frame(
        &chain.genesis,
        2,
        Some(chain.frames[0].id.clone()),
        chain.commits[1].target.application.unwrap().0.0,
    );
    wrong_next.header.version.as_mut().unwrap().app += 1;
    native::resign(&mut wrong_next);
    assert_eq!(
        chain::reject(&mut verifier, &wrong_next),
        FinalityError::WrongNativeApplicationVersion
    );
    chain::accept(&mut verifier, &chain.frames[1]);
    assert!(authenticate_current_application_version(&verifier, &chain.commits[1].target).is_ok());
}

#[test]
fn scheduled_upgrade_stops_at_activation_without_mutating_previous_history() {
    let mut genesis = genesis::genesis();
    genesis.upgrades.push(ScheduledUpgrade {
        activation_height: 2,
        protocol_version: 2,
        profile: SecurityProfile::ClassicalDev,
        code_digest: B256::repeat_byte(0x22),
        migration_id: Bytes::from_static(b"test-only-unsupported-upgrade"),
    });
    let initial = initialize_development_state(&genesis, &development_state_budget()).unwrap();
    let mut verifier = chain::verifier(&genesis);
    let first = native::frame(&genesis, 1, None, initial.target.content_digest.0);
    chain::accept(&mut verifier, &first);
    let activation = native::frame(&genesis, 2, Some(first.id), [0x55; 32]);
    assert_eq!(
        chain::reject(&mut verifier, &activation),
        FinalityError::UnsupportedActivation
    );
    assert_eq!(verifier.height(), 1);
}

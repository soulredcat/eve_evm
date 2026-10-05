// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{chain, genesis, native};
use eve_consensus_comet::consensus::history::HistoryError;
use eve_finality_verifier::{
    FinalityError, authenticate_current_application_version, initialize_development_finality,
};
use eve_state::{SecurityProfile, development_state_budget, initialize_development_state};

#[test]
fn d41_first_header_binds_actual_canonical_genesis_not_just_chain_labels() {
    let original = genesis::genesis();
    let initial = initialize_development_state(&original, &development_state_budget()).unwrap();
    let mut changed = original.clone();
    changed.initial_timestamp += 1;
    let foreign = initialize_development_state(&changed, &development_state_budget()).unwrap();
    assert_eq!(original.network_name, changed.network_name);
    assert_eq!(original.evm_chain_id, changed.evm_chain_id);
    assert_ne!(
        initial.target.identity.genesis,
        foreign.target.identity.genesis
    );
    assert_ne!(initial.target.content_digest, foreign.target.content_digest);
    let mut verifier = chain::verifier(&original);
    assert_eq!(verifier.identity(), &initial.target.identity);
    let wrong = native::frame(&original, 1, None, foreign.target.content_digest.0);
    assert_eq!(
        chain::reject(&mut verifier, &wrong),
        FinalityError::Native(HistoryError::WrongGenesisApplicationHash)
    );
    chain::accept(
        &mut verifier,
        &native::frame(&original, 1, None, initial.target.content_digest.0),
    );
    assert_eq!(
        authenticate_current_application_version(&verifier, &initial.target).err(),
        Some(FinalityError::WrongApplicationHeight)
    );
}

#[test]
fn unsupported_profile_genesis_never_creates_a_classical_verifier() {
    let mut genesis = genesis::genesis();
    genesis.profile = SecurityProfile::HybridExperimental;
    assert!(initialize_development_finality(&genesis, &development_state_budget()).is_err());
    genesis.profile = SecurityProfile::PqProfileVerified;
    assert!(initialize_development_finality(&genesis, &development_state_budget()).is_err());
}

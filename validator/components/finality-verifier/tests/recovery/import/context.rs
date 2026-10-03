// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_finality_verifier::{
    ImportError, imported_state_commit, initialize_authenticated_import,
    prepare_authenticated_import,
};
use eve_state::{Address, B256, development_state_budget};

use super::support::{bind_current_hash, input};
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn execution_time_proposer_mix_hash_base_fee_gas_limit_and_parent_are_certified_context() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..6 {
        let mut delta = input(&chain, 1);
        let header = &mut delta.execution.header;
        match field {
            0 => header.timestamp += 1,
            1 => header.beneficiary = Address::repeat_byte(0x81),
            2 => header.mix_hash = B256::repeat_byte(0x82),
            3 => header.base_fee_per_gas = Some(header.base_fee_per_gas.unwrap() + 1),
            4 => header.gas_limit -= 1,
            5 => header.parent_hash = B256::repeat_byte(0x83),
            _ => unreachable!(),
        }
        bind_current_hash(&mut delta);
        assert_eq!(
            prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES)
                .unwrap_err(),
            ImportError::InvalidExecutionContext,
        );
    }
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn import_height_range_is_exact_h_h_plus_one_and_never_a_peer_selected_gap() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..4 {
        let mut delta = input(&chain, 1);
        match field {
            0 => delta.journal.target_height = u64::MAX,
            1 => delta.execution.header.number = 2,
            2 => delta.finalized.header.height = 2,
            3 => delta.lookahead.frame.header.height = 3,
            _ => unreachable!(),
        }
        assert_eq!(
            prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES)
                .unwrap_err(),
            ImportError::WrongHeight,
        );
    }
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

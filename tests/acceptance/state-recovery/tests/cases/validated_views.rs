// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support;

use alloy_primitives::{Address, U256};
use eve_state::{
    capture_state_view, development_state_budget, read_account, read_code, read_execution_hash,
    read_storage,
};
use support::{commits::structural_genesis, execution::executed_commit};

#[test]
fn ts01_validated_views_distinguish_known_absence_from_incomplete_code_or_history() {
    let parent = structural_genesis("two_slot_contract");
    let view = capture_state_view(
        parent.state.clone(),
        parent.target.clone(),
        &development_state_budget(),
    )
    .unwrap();
    assert_eq!(read_account(&view, Address::repeat_byte(0x99)), None);
    assert_eq!(
        read_storage(&view, Address::repeat_byte(0x99), U256::ZERO),
        U256::ZERO
    );
    let code = parent.state.accounts[&Address::repeat_byte(0x11)].code_hash;
    assert_eq!(read_code(&view, code).unwrap(), parent.state.codes[&code]);
    let (genesis, current) = executed_commit();
    let current_view = capture_state_view(
        current.state.clone(),
        current.target.clone(),
        &development_state_budget(),
    )
    .unwrap();
    assert_eq!(
        read_execution_hash(&current_view, 2, 0).unwrap(),
        Some(genesis.target.execution_hash)
    );
    assert_eq!(read_execution_hash(&current_view, 2, 2).unwrap(), None);
    let mut missing = current.state.clone();
    missing.block_hashes.remove(&0);
    assert!(capture_state_view(missing, current.target, &development_state_budget()).is_err());
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_development_fixtures::{
    recovery::{recovery_chain, recovery_chain_with_nonempty_tail, sender, signed_transaction},
    transactions::signed_transaction_with_nonce,
};
use eve_evm::{decode_signed_transaction, split_collected_fees};
use eve_state::{Address, SystemValue, U256, development_state_budget, validate_state_commit};
#[test]
fn maintained_nonce_signer_preserves_exact_nonce_zero_golden_and_sender_identity() {
    assert_eq!(signed_transaction_with_nonce(0), signed_transaction());
    for nonce in 0..=1 {
        let bytes = signed_transaction_with_nonce(nonce);
        let decoded = decode_signed_transaction(&bytes, 31_337, 131_072).unwrap();
        assert_eq!(decoded.sender(), sender());
        assert_eq!(decoded.evm().nonce, nonce);
    }
}
#[test]
fn nonempty_tail_has_two_signed_effects_exact_roots_receipts_nonce_and_fee_conservation() {
    let original = recovery_chain();
    let chain = recovery_chain_with_nonempty_tail();
    assert_eq!(original.commits[..=1], chain.commits[..=1]);
    assert_eq!(original.frames[0].header, chain.frames[0].header);
    assert_eq!(original.frames[0].commit, chain.frames[0].commit);
    for height in 1..=2 {
        let commit = &chain.commits[height];
        assert_eq!(
            commit.block.transactions,
            vec![signed_transaction_with_nonce((height - 1) as u64)]
        );
        assert_eq!(commit.block.receipts.len(), 1);
        assert_eq!(commit.state.accounts[&sender()].nonce, height as u64);
        validate_state_commit(commit, &development_state_budget()).unwrap();
    }
    assert!(original.commits[2].block.transactions.is_empty());
    assert!(chain.commits[3].block.transactions.is_empty());
    assert_eq!(
        chain.commits[3].state.accounts,
        chain.commits[2].state.accounts
    );
    assert_eq!(chain.commits[3].state.system, chain.commits[2].state.system);
    assert_eq!(
        chain.commits[2].state.accounts[&Address::with_last_byte(0x42)].storage[&U256::ZERO],
        U256::from(99)
    );
    let collected =
        U256::from(chain.commits[1].block.header.gas_used + chain.commits[2].block.header.gas_used)
            * U256::from(2_000_000_000_u64);
    let expected = split_collected_fees(collected);
    let ledger = chain.commits[2]
        .state
        .system
        .values()
        .find_map(|record| match record.value {
            SystemValue::Fee {
                burned,
                node_pool,
                validator_pool,
            } => Some((burned, node_pool, validator_pool)),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        ledger,
        (expected.burn, expected.node_pool, expected.validator_pool)
    );
    assert_eq!(ledger.0 + ledger.1 + ledger.2, collected);
    assert_eq!(
        chain.commits[0].state.accounts[&sender()].balance
            - chain.commits[2].state.accounts[&sender()].balance,
        collected
    );
}

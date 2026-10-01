// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use std::collections::BTreeMap;

use alloy_primitives::{KECCAK256_EMPTY, keccak256};
use alloy_trie::{EMPTY_ROOT_HASH, Nibbles, proof::verify_proof};
use eve_state::{
    Address, B256, BlockPayload, Bytes, ProofLimits, StateAccount, StateProofError, StateView,
    U256, build_account_proof, build_state_commit, capture_state_view, compute_evm_root,
    development_state_budget, estimate_proof_reservation,
};

fn view() -> StateView {
    // Structural complete view; literal oracle is the reviewed EthereumJS B1 fixture.
    let parent = support::genesis();
    let mut state = parent.state;
    let code = Bytes::from_static(&[0x60, 0x63, 0x60, 0x00, 0x55, 0x00]);
    let code_hash = keccak256(&code);
    state.codes = BTreeMap::from([(code_hash, code)]);
    state.accounts = BTreeMap::from([(
        Address::repeat_byte(0x11),
        StateAccount {
            nonce: 1,
            balance: U256::from(100),
            code_hash,
            storage: BTreeMap::from([(U256::ZERO, U256::from(7)), (U256::from(1), U256::from(11))]),
        },
    )]);
    let expected: B256 = "ae0d6b7df1d6919435d7eb299fdb052baf3ff28f1323bf178aae1beab114e86e"
        .parse()
        .unwrap();
    assert_eq!(compute_evm_root(&state.accounts).0, expected);
    let mut header = parent.block.header;
    header.state_root = expected;
    let budget = development_state_budget();
    let commit = build_state_commit(
        None,
        state,
        BlockPayload {
            header,
            transactions: vec![],
            receipts: vec![],
        },
        &budget,
    )
    .unwrap();
    capture_state_view(commit.state, commit.target, &budget).unwrap()
}

fn limits() -> ProofLimits {
    ProofLimits {
        maximum_requested_slots: 64,
        maximum_proof_bytes: 262_144,
        maximum_rebuild_bytes: 64 * 1_048_576,
    }
}

#[test]
fn account_and_storage_inclusion_absence_proofs_match_independent_literal_roots() {
    let view = view();
    let address = Address::repeat_byte(0x11);
    let result = build_account_proof(
        &view,
        address,
        &[U256::ZERO, U256::from(2)],
        &limits(),
        64 * 1_048_576,
    )
    .unwrap();
    assert_eq!(
        result.storage_root,
        "1f2c9cd09733bf204ea8259127b0fd69b2b994e8ec2b80eee945b565bebf8b41"
            .parse::<B256>()
            .unwrap()
    );
    let leaf = hex::decode("f8440164a01f2c9cd09733bf204ea8259127b0fd69b2b994e8ec2b80eee945b565bebf8b41a064b512fb5ef6061ab79865e25ba249f218fd506ace73f4a2aa5054c038d9b427").unwrap();
    verify_proof(
        result.state_root.0,
        Nibbles::unpack(keccak256(address)),
        Some(leaf),
        &result.account_proof,
    )
    .unwrap();
    assert_eq!(result.storage_proof[0].value, U256::from(7));
    verify_proof(
        result.storage_root,
        Nibbles::unpack(keccak256(U256::ZERO.to_be_bytes::<32>())),
        Some(vec![7]),
        &result.storage_proof[0].proof,
    )
    .unwrap();
    assert_eq!(result.storage_proof[1].value, U256::ZERO);
    verify_proof(
        result.storage_root,
        Nibbles::unpack(keccak256(U256::from(2).to_be_bytes::<32>())),
        None,
        &result.storage_proof[1].proof,
    )
    .unwrap();
    let absent = build_account_proof(
        &view,
        Address::repeat_byte(0x22),
        &[U256::ZERO],
        &limits(),
        64 * 1_048_576,
    )
    .unwrap();
    assert_eq!(absent.code_hash, KECCAK256_EMPTY);
    assert_eq!(absent.storage_root, EMPTY_ROOT_HASH);
    assert_eq!((absent.nonce, absent.balance), (0, U256::ZERO));
    verify_proof(
        absent.state_root.0,
        Nibbles::unpack(keccak256(absent.address)),
        None,
        &absent.account_proof,
    )
    .unwrap();
}

#[test]
fn proof_tampering_truncation_wrong_key_and_resource_limits_reject() {
    let view = view();
    let address = Address::repeat_byte(0x11);
    let result =
        build_account_proof(&view, address, &[U256::ZERO], &limits(), 64 * 1_048_576).unwrap();
    let mut proof = result.storage_proof[0].proof.clone();
    proof.pop();
    assert!(
        verify_proof(
            result.storage_root,
            Nibbles::unpack(keccak256(U256::ZERO.to_be_bytes::<32>())),
            Some(vec![7]),
            &proof
        )
        .is_err()
    );
    let mut tampered = result.account_proof[0].to_vec();
    tampered[0] ^= 1;
    assert!(
        verify_proof(
            result.state_root.0,
            Nibbles::unpack(keccak256(address)),
            None,
            &[Bytes::from(tampered)]
        )
        .is_err()
    );
    assert!(
        verify_proof(
            result.storage_root,
            Nibbles::unpack(keccak256(U256::from(1).to_be_bytes::<32>())),
            Some(vec![7]),
            &result.storage_proof[0].proof
        )
        .is_err()
    );
    assert!(matches!(
        build_account_proof(&view, address, &[], &limits(), 1),
        Err(StateProofError::Reservation { .. })
    ));
    assert!(matches!(
        build_account_proof(
            &view,
            address,
            &[U256::ZERO, U256::ZERO],
            &limits(),
            64 * 1_048_576
        ),
        Err(StateProofError::Limit(_))
    ));
    let tiny = ProofLimits {
        maximum_proof_bytes: 1,
        ..limits()
    };
    assert!(matches!(
        build_account_proof(&view, address, &[], &tiny, 64 * 1_048_576),
        Err(StateProofError::Limit(_))
    ));
    assert!(estimate_proof_reservation(&view, usize::MAX).is_err());
}

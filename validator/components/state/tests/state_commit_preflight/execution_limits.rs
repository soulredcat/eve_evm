// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::fixtures::{fields, fixture, list};
use eve_state::{StateError, preflight_state_commit};

#[test]
fn raw_execution_item_aggregate_and_count_limits_are_checked_before_owned_decode() {
    let (_, bytes, budget) = fixture();
    let mut commit = fields(&bytes);
    let mut block = fields(&commit[4]);
    for (transaction_bytes, receipt_bytes, accepted) in [
        (131_072, 1, true),
        (131_073, 1, false),
        (1, 4_194_304, true),
        (1, 4_194_305, false),
    ] {
        block[1] = list(&[alloy_rlp::encode(
            vec![0x31_u8; transaction_bytes].as_slice(),
        )]);
        block[2] = list(&[alloy_rlp::encode(vec![0x31_u8; receipt_bytes].as_slice())]);
        commit[4] = list(&block);
        let altered = list(&commit);
        assert_eq!(preflight_state_commit(&altered, &budget).is_ok(), accepted);
    }
    let small = alloy_rlp::encode([0xc0_u8].as_slice());
    block[1] = list(&[small.clone(), small.clone()]);
    block[2] = list(&[small.clone(), small]);
    commit[4] = list(&block);
    let mut bounded = budget;
    bounded.maximum_journal_operations = 1;
    assert!(matches!(
        preflight_state_commit(&list(&commit), &bounded),
        Err(StateError::BudgetExceeded)
    ));
    // The maintained decoder's shared transaction+receipt raw envelope is 8 MiB.
    let transaction = alloy_rlp::encode(vec![0x31_u8; 131_072].as_slice());
    block[1] = list(&vec![transaction; 65]);
    block[2] = list(&vec![alloy_rlp::encode([0xc0_u8].as_slice()); 65]);
    commit[4] = list(&block);
    assert!(matches!(
        preflight_state_commit(&list(&commit), &budget),
        Err(StateError::BudgetExceeded)
    ));
}

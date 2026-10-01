// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::corpus::{
    load_pinned_cases::load_pinned_cases, materialize_state::materialize_state,
    project_state::project_state, run_reference_case::run_reference_case,
};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::keccak256;
use eve_evm::compute_state_root;

#[test]
fn te02_all_3495_shanghai_cases_match_unchanged_vanilla_roots_and_receipts() {
    let mut executed = 0;
    let mut rejected = 0;
    for (name, case) in load_pinned_cases() {
        let post = &case.post["Shanghai"][0];
        assert_eq!(
            compute_state_root(&materialize_state(&post.state)),
            post.hash,
            "Expected fixture root {name}"
        );
        match run_reference_case(&case) {
            Err(actual) => {
                let expected = post
                    .expected_exception
                    .as_ref()
                    .unwrap_or_else(|| panic!("{name}: {actual}"));
                assert!(
                    expected.split('|').any(|category| category == actual),
                    "{name}: {actual} versus {expected}"
                );
                assert!(post.receipt.is_none(), "{name}");
                rejected += 1;
            }
            Ok((state, execution, receipt, _)) => {
                assert!(post.expected_exception.is_none(), "{name}");
                let mut expected = post.state.clone();
                for account in expected.values_mut() {
                    account.storage.retain(|_, value| !value.is_zero());
                }
                assert_eq!(project_state(&state), expected, "Complete poststate {name}");
                assert_eq!(compute_state_root(&state), post.hash, "Vanilla root {name}");
                assert_eq!(
                    receipt.encoded_2718(),
                    post.receipt.as_ref().unwrap().rlp,
                    "Receipt {name}"
                );
                assert_eq!(
                    keccak256(alloy_rlp::encode(execution.logs().to_vec())),
                    post.logs,
                    "Logs hash {name}"
                );
                executed += 1;
            }
        }
    }
    assert_eq!((executed, rejected), (3202, 293));
    println!(
        "Pinned Shanghai vanilla reference: 3495 cases, 3202 executed, 293 expected invalid; exact upstream outputs unchanged; 120M reference environment, not the 30M development profile."
    );
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::tests::import_fixtures::ImportChain;
use eve_development_fixtures::recovery::recovery_chain_with_nonempty_tail;
use eve_finality_verifier::{
    AuthenticatedImportInput, NativeDataFrame, NativeFrame, encode_logical_import_wire,
    imported_state_commit, initialize_authenticated_import, into_imported_state,
    preflight_logical_import_wire, prepare_authenticated_import,
};
use eve_state::{
    development_state_budget, estimate_journal_candidate_reservation, project_state_journal,
};
use std::sync::Arc;

/// Shared canonical executor/signers supply all data; this adapter only builds maintained import DTOs/codecs.
pub(super) fn nonempty_import_chain() -> ImportChain {
    let chain = recovery_chain_with_nonempty_tail();
    let budget = development_state_budget();
    let frames: Vec<_> = chain
        .frames
        .iter()
        .map(|frame| NativeFrame {
            block_id: frame.id.clone(),
            header: frame.header.clone(),
            commit: frame.commit.clone(),
        })
        .collect();
    let inputs: Vec<_> = (1..=2)
        .map(|height| AuthenticatedImportInput {
            journal: project_state_journal(
                &chain.commits[height - 1].state,
                &chain.commits[height - 1].target,
                &chain.commits[height].state,
                height as u64,
                &budget,
            )
            .unwrap(),
            execution: chain.commits[height].block.clone(),
            finalized: frames[height - 1].clone(),
            lookahead: NativeDataFrame {
                frame: frames[height].clone(),
                transactions: chain.commits[height + 1].block.transactions.clone(),
            },
        })
        .collect();
    let mut parent = Arc::new(initialize_authenticated_import(&chain.genesis, &budget).unwrap());
    let mut records = Vec::new();
    for (index, input) in inputs.iter().enumerate() {
        let required = estimate_journal_candidate_reservation(
            &imported_state_commit(&parent).state,
            &input.journal,
            &budget,
        )
        .unwrap();
        parent = into_imported_state(
            prepare_authenticated_import(&parent, Arc::new(input.clone()), &budget, required)
                .unwrap(),
        );
        assert_eq!(imported_state_commit(&parent), &chain.commits[index + 1]);
        let bytes = encode_logical_import_wire(input, &budget).unwrap();
        preflight_logical_import_wire(&bytes, &budget).unwrap();
        records.push(bytes);
    }
    assert_eq!(
        chain.commits[1].state.accounts[&eve_development_fixtures::recovery::sender()].nonce,
        1
    );
    assert_eq!(
        chain.commits[2].state.accounts[&eve_development_fixtures::recovery::sender()].nonce,
        2
    );
    assert!(
        chain.commits[1..=2]
            .iter()
            .all(|commit| commit.block.receipts.len() == 1)
    );
    ImportChain {
        genesis: chain.genesis,
        commits: chain.commits,
        inputs,
        records,
    }
}

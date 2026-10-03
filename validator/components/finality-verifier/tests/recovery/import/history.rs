// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use eve_finality_verifier::{
    ImportError, imported_state_commit, initialize_authenticated_import,
    prepare_authenticated_import,
};
use eve_state::{B256, ExecutionBlockHash, JournalOperation, development_state_budget};

use super::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn absent_duplicate_wrong_current_or_old_execution_hash_entries_are_rejected_before_staging() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..4 {
        let mut delta = input(&chain, 1);
        let index = delta
            .journal
            .operations
            .iter()
            .position(|operation| {
                matches!(operation, JournalOperation::SetExecutionBlockHash { .. })
            })
            .unwrap();
        match field {
            0 => {
                delta.journal.operations.remove(index);
            }
            1 => delta
                .journal
                .operations
                .push(delta.journal.operations[index].clone()),
            2 => {
                if let JournalOperation::SetExecutionBlockHash { hash, .. } =
                    &mut delta.journal.operations[index]
                {
                    *hash = ExecutionBlockHash(B256::repeat_byte(0x91));
                }
            }
            3 => {
                if let JournalOperation::SetExecutionBlockHash { height, .. } =
                    &mut delta.journal.operations[index]
                {
                    *height = 0;
                }
            }
            _ => unreachable!(),
        }
        assert_eq!(
            prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES)
                .unwrap_err(),
            ImportError::InvalidExecutionHistory,
        );
    }
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

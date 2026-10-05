// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_finality_verifier::AuthenticatedImportInput;
use eve_state::{
    ExecutionBlockHash, JournalOperation, development_state_budget, project_state_journal,
};

use crate::recovery_support::{RecoveryChain, envelope};

pub fn input(chain: &RecoveryChain, height: usize) -> AuthenticatedImportInput {
    let envelope = envelope(chain, height);
    AuthenticatedImportInput {
        journal: project_state_journal(
            &chain.commits[height - 1].state,
            &chain.commits[height - 1].target,
            &chain.commits[height].state,
            u64::try_from(height).unwrap(),
            &development_state_budget(),
        )
        .unwrap(),
        execution: envelope.execution,
        finalized: envelope.finalized,
        lookahead: envelope.lookahead,
    }
}

/// Context mutation tests bind the supplied history entry to the changed header
/// so a prior history rejection cannot mask the intended context check.
pub fn bind_current_hash(input: &mut AuthenticatedImportInput) {
    let current = ExecutionBlockHash(input.execution.header.hash_slow());
    for operation in &mut input.journal.operations {
        if let JournalOperation::SetExecutionBlockHash { hash, .. } = operation {
            *hash = current;
        }
    }
}

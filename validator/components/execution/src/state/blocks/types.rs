// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256};
use eve_state::{StateCommit, StateJournal};

use crate::{FeeAllocation, TransactionOutcome};

/// Agreed caller inputs; accepting them does not authenticate consensus authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutionBlockInput {
    pub timestamp: u64,
    pub proposer: Address,
    pub previous_consensus_hash: B256,
}

#[derive(Debug)]
pub struct PreparedStateBlock {
    pub commit: StateCommit,
    pub journal: StateJournal,
    pub outcomes: Vec<TransactionOutcome>,
    pub fees: FeeAllocation,
}

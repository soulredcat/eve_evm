// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::TestFixture;
use eve_state::{Address, B256, development_state_budget};
use eve_storage::state::{commit_state, read_state_service};

pub(in crate::consensus) fn advance_state(fixture: &mut TestFixture) {
    let parent = read_state_service(&fixture.service).unwrap();
    let prepared = eve_evm::execute_state_block(
        parent.commit(),
        &eve_evm::ExecutionBlockInput {
            timestamp: parent.commit().target.timestamp + 1,
            proposer: Address::repeat_byte(1),
            previous_consensus_hash: B256::repeat_byte(8),
        },
        &[],
        &development_state_budget(),
        256 * 1024 * 1024,
    )
    .unwrap();
    commit_state(&mut fixture.store, &prepared.commit).unwrap();
}

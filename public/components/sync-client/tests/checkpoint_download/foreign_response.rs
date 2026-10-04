// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    BlockPayload, build_state_version, development_state_budget, initialize_development_state,
};
use eve_storage::checkpoints::messages::CheckpointResponse;

/// Canonical metadata for an independently initialized foreign development identity.
/// This structural fixture carries no execution certificate or checkpoint authority.
pub(super) fn foreign_execution_response() -> CheckpointResponse {
    let mut genesis = eve_development_fixtures::genesis::genesis();
    // The development profile fixes its network name; another valid genesis is
    // still a foreign chain identity and must never match the configured anchor.
    genesis.accounts[0].nonce += 1;
    let budget = development_state_budget();
    let initial = initialize_development_state(&genesis, &budget).unwrap();
    let mut header = initial.block.header.clone();
    header.number = 1;
    header.timestamp += 1;
    header.parent_hash = initial.target.execution_hash.0;
    let target = build_state_version(&initial.state, &header, &budget).unwrap();
    let block = BlockPayload {
        header,
        transactions: Vec::new(),
        receipts: Vec::new(),
    };
    CheckpointResponse::Execution {
        target,
        block: Box::new(block),
    }
}

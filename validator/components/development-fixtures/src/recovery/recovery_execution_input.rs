// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native::Frame;
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_evm::ExecutionBlockInput;
use eve_state::{B256, DevelopmentGenesis};

pub(super) fn recovery_execution_input(
    genesis: &DevelopmentGenesis,
    frame: &Frame,
) -> ExecutionBlockInput {
    for validator in &genesis.validators {
        if validator_address(&validator.classical_public_key).as_slice()
            == frame.header.proposer_address
        {
            let previous_consensus_hash = match &frame.header.last_block_id {
                Some(id) => B256::from_slice(&id.hash),
                None => B256::ZERO,
            };
            return ExecutionBlockInput {
                timestamp: u64::try_from(frame.header.time.unwrap().seconds).unwrap(),
                proposer: validator.owner,
                previous_consensus_hash,
            };
        }
    }
    panic!("fixture proposer must belong to the deterministic genesis validator set")
}

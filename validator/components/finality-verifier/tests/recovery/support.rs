// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{genesis, native};
use eve_consensus_comet::consensus::certificates::{hash_transaction_data, validator_address};
use eve_evm::{ExecutionBlockInput, decode_signed_transaction, execute_state_block};
use eve_finality_verifier::{CompactRecoveryEnvelopeV1, NativeDataFrame, NativeFrame};
use eve_state::{
    Address, B256, Bytes, DevelopmentGenesis, GenesisAccount, StateCommit, U256,
    development_state_budget, initialize_development_state,
};

pub const CLONE_BYTES: usize = 512 * 1_048_576;

pub struct RecoveryChain {
    pub genesis: DevelopmentGenesis,
    pub commits: Vec<StateCommit>,
    pub frames: Vec<native::Frame>,
}

/// The literal signed bytes are the existing state-recovery trie-v1 execution vector.
/// Its deterministic sender and all native signing keys are unsafe test identities.
pub fn signed_transaction() -> Bytes {
    "0xf866808477359400830186a0940000000000000000000000000000000000000042808082f4f5a02c1d1a7db8b28ed638c4c0a70b8789ae4fc8d41fbf881cca9ccb0a97f8f487aba00a5013ea38ccecec4dab4c2c82674109c7a303ad69772b56ef82a47eef05d4c6"
        .parse()
        .unwrap()
}

pub fn sender() -> Address {
    decode_signed_transaction(&signed_transaction(), 31_337, 131_072)
        .unwrap()
        .sender()
}

pub fn funded_genesis() -> DevelopmentGenesis {
    let mut genesis = genesis::genesis();
    genesis.accounts.push(GenesisAccount {
        address: sender(),
        funded_balance: U256::from(10_u64).pow(U256::from(22)),
        nonce: 0,
        code: Bytes::new(),
    });
    genesis.accounts.push(GenesisAccount {
        address: Address::with_last_byte(0x42),
        funded_balance: U256::ZERO,
        nonce: 0,
        code: "0x606360005500".parse().unwrap(),
    });
    genesis
}

pub fn recovery_chain() -> RecoveryChain {
    let genesis = funded_genesis();
    let initial = initialize_development_state(&genesis, &development_state_budget()).unwrap();
    let mut commits = vec![initial];
    let mut frames = Vec::new();
    for height in 1..=3 {
        let parent = commits.last().unwrap();
        let application = if height == 1 {
            parent.target.content_digest.0
        } else {
            parent.target.application.unwrap().0.0
        };
        let mut frame = native::frame(
            &genesis,
            height,
            frames
                .last()
                .map(|previous: &native::Frame| previous.id.clone()),
            application,
        );
        let transactions = if height == 1 {
            vec![signed_transaction()]
        } else {
            Vec::new()
        };
        frame.header.data_hash = hash_transaction_data(&transactions).unwrap().to_vec();
        native::resign(&mut frame);
        let proposer = genesis
            .validators
            .iter()
            .find(|validator| {
                validator_address(&validator.classical_public_key).as_slice()
                    == frame.header.proposer_address
            })
            .unwrap()
            .owner;
        let prepared = execute_state_block(
            parent,
            &ExecutionBlockInput {
                timestamp: u64::try_from(frame.header.time.unwrap().seconds).unwrap(),
                proposer,
                previous_consensus_hash: frame
                    .header
                    .last_block_id
                    .as_ref()
                    .map_or(B256::ZERO, |id| B256::from_slice(&id.hash)),
            },
            &transactions,
            &development_state_budget(),
            CLONE_BYTES,
        )
        .unwrap();
        commits.push(prepared.commit);
        frames.push(frame);
    }
    RecoveryChain {
        genesis,
        commits,
        frames,
    }
}

pub fn native_frame(frame: &native::Frame) -> NativeFrame {
    NativeFrame {
        block_id: frame.id.clone(),
        header: frame.header.clone(),
        commit: frame.commit.clone(),
    }
}

pub fn envelope(chain: &RecoveryChain, height: usize) -> CompactRecoveryEnvelopeV1 {
    CompactRecoveryEnvelopeV1 {
        parent: chain.commits[height - 1].target.clone(),
        expected: chain.commits[height].target.clone(),
        execution: chain.commits[height].block.clone(),
        finalized: native_frame(&chain.frames[height - 1]),
        lookahead: NativeDataFrame {
            frame: native_frame(&chain.frames[height]),
            transactions: chain.commits[height + 1].block.transactions.clone(),
        },
    }
}

pub fn resign(frame: &mut NativeFrame, genesis: &DevelopmentGenesis) {
    let mut native = native::frame(genesis, frame.header.height, None, [0; 32]);
    native.header = frame.header.clone();
    native::resign(&mut native);
    *frame = native_frame(&native);
}

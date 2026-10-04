// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native;
pub use eve_development_fixtures::recovery::{CLONE_BYTES, RecoveryChain, recovery_chain};
use eve_finality_verifier::{CompactRecoveryEnvelopeV1, NativeDataFrame, NativeFrame};
use eve_state::DevelopmentGenesis;

pub fn funded_genesis() -> DevelopmentGenesis {
    eve_development_fixtures::recovery::funded_genesis()
}

pub fn sender() -> eve_state::Address {
    eve_development_fixtures::recovery::sender()
}

pub fn signed_transaction() -> eve_state::Bytes {
    eve_development_fixtures::recovery::signed_transaction()
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

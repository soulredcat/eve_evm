// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_development_fixtures::native::{self, Frame};
use eve_finality_verifier::{CompactRecoveryEnvelopeV1, NativeDataFrame, NativeFrame};
use eve_state::DevelopmentGenesis;

pub(super) use eve_development_fixtures::recovery::{RecoveryChain, recovery_chain};

fn native_frame(frame: &Frame) -> NativeFrame {
    NativeFrame {
        block_id: frame.id.clone(),
        header: frame.header.clone(),
        commit: frame.commit.clone(),
    }
}

/// Convert the shared actual native/execution fixture into this consumer's wire DTO.
pub(super) fn envelope(chain: &RecoveryChain, height: usize) -> CompactRecoveryEnvelopeV1 {
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

/// Test DTO conversion delegates every native signature operation to the shared fixture.
pub(super) fn resign(frame: &mut NativeFrame, genesis: &DevelopmentGenesis) {
    let mut source = native::frame(genesis, frame.header.height, None, [0; 32]);
    source.header = frame.header.clone();
    native::resign(&mut source);
    *frame = native_frame(&source);
}

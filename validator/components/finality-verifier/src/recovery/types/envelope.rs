// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::tendermint::types::{BlockId, Commit, Header};
use eve_state::{BlockPayload, Bytes, StateVersion};

/// Untrusted inputs. Construction establishes neither execution nor authentication.
#[derive(Clone, Debug, PartialEq)]
pub struct CompactRecoveryEnvelopeV1 {
    pub parent: StateVersion,
    pub expected: StateVersion,
    pub execution: BlockPayload,
    pub finalized: NativeFrame,
    pub lookahead: NativeDataFrame,
}

/// Native proof inputs; the applicable set comes from local canonical genesis.
#[derive(Clone, Debug, PartialEq)]
pub struct NativeFrame {
    pub block_id: BlockId,
    pub header: Header,
    pub commit: Commit,
}

/// Certified lookahead data must also match the native header's Data.Hash.
#[derive(Clone, Debug, PartialEq)]
pub struct NativeDataFrame {
    pub frame: NativeFrame,
    pub transactions: Vec<Bytes>,
}

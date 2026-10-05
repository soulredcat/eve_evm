// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{NativeFieldRule, NativeMessageKind};
use crate::recovery::RecoveryError;

/// Exact field topology from the pinned CometBFT native messages, not a decoder.
pub(crate) fn native_field_rule(
    kind: NativeMessageKind,
    field: u64,
) -> Result<NativeFieldRule, RecoveryError> {
    use NativeFieldRule::{Bytes, Message, Signatures, Varint};
    use NativeMessageKind::{
        BlockId, Commit, CommitSig, Consensus, Header, PartSetHeader, Timestamp,
    };
    let rule = match (kind, field) {
        (BlockId, 1) | (PartSetHeader, 2) => Bytes(32),
        (BlockId, 2) => Message(PartSetHeader, 64),
        (PartSetHeader, 1)
        | (Consensus | Timestamp, 1 | 2)
        | (Header, 3)
        | (Commit, 1 | 2)
        | (CommitSig, 1) => Varint,
        (Header, 1) => Message(Consensus, 24),
        (Header, 2) => Bytes(50),
        (Header, 4) | (CommitSig, 3) => Message(Timestamp, 24),
        (Header, 5) | (Commit, 3) => Message(BlockId, 128),
        (Header, 6..=13) => Bytes(32),
        (Header, 14) | (CommitSig, 2) => Bytes(20),
        (Commit, 4) => Signatures,
        (CommitSig, 4) => Bytes(64),
        _ => return Err(RecoveryError::NonCanonicalEncoding),
    };
    Ok(rule)
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Safe categories: errors never contain public keys, signatures or raw messages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SigningError {
    InvalidChainId,
    InvalidHeight,
    InvalidRound,
    InvalidMessageType,
    InvalidTimestamp,
    InvalidBlockId,
    InvalidValidatorAddress,
    InvalidValidatorIndex,
    InvalidSignatureLength,
    UnsupportedVoteExtension,
}

pub(crate) const GO_ZERO_TIME_SECONDS: i64 = -62_135_596_800;
pub(crate) const MAX_TIMESTAMP_SECONDS: i64 = 253_402_300_799;
pub(crate) const MAX_CHAIN_ID_BYTES: usize = 50;

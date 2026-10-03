// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// This schema is intentionally bounded by the existing opaque recovery budget.
pub(crate) const RECOVERY_DOMAIN: &[u8] = b"EVE_RECOVERY_V1";
pub(crate) const MAXIMUM_RECOVERY_BYTES: usize = 4_198_312;
pub(crate) const MAXIMUM_VERSION_BYTES: usize = 4_096;
pub(crate) const MAXIMUM_NATIVE_FRAME_BYTES: usize = 16_384;
pub(crate) const MAXIMUM_BLOCK_ID_BYTES: usize = 128;
pub(crate) const MAXIMUM_NATIVE_HEADER_BYTES: usize = 4_096;
pub(crate) const MAXIMUM_EXECUTION_HEADER_BYTES: usize = 4_096;
pub(crate) const MAXIMUM_NATIVE_COMMIT_BYTES: usize = 8_192;
pub(crate) const MAXIMUM_TRANSACTION_BYTES: usize = 131_072;
pub(crate) const MAXIMUM_TRANSACTIONS: usize = 30_000_000 / 21_000;
pub(crate) const MAXIMUM_NATIVE_DATA_BYTES: usize = 4_194_304;
pub(crate) const MAXIMUM_RECEIPT_BYTES: usize = 4_194_304;
pub(crate) const MAXIMUM_EXECUTION_DATA_BYTES: usize = 8_388_608;

#[derive(Clone, Copy)]
pub(crate) enum NativeMessageKind {
    BlockId,
    PartSetHeader,
    Header,
    Consensus,
    Timestamp,
    Commit,
    CommitSig,
}

#[derive(Clone, Copy)]
pub(crate) enum NativeFieldRule {
    Varint,
    Bytes(usize),
    Message(NativeMessageKind, usize),
    Signatures,
}

pub(crate) struct RecoverySlices<'a> {
    pub(crate) parent: &'a [u8],
    pub(crate) expected: &'a [u8],
    pub(crate) execution: &'a [u8],
    pub(crate) finalized: &'a [u8],
    pub(crate) lookahead: &'a [u8],
}

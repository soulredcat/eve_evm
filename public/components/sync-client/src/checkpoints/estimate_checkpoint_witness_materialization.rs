// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result};
use eve_finality_verifier::NativeDataFrame;
use prost::Message;

/// Borrowed actual protobuf sizes/counts and metadata bytes bound codec copies.
/// Canonical wire measurement still validates maintained grammar after this real lease.
pub(super) fn estimate_checkpoint_witness_materialization(
    native: &NativeDataFrame,
    metadata_bytes: usize,
) -> Result<usize> {
    let protobuf = native
        .frame
        .block_id
        .encoded_len()
        .checked_add(native.frame.header.encoded_len())
        .and_then(|bytes| bytes.checked_add(native.frame.commit.encoded_len()))
        .context("SYNC_CHECKPOINT_RESERVATION_OVERFLOW")?;
    let transactions = native
        .transactions
        .iter()
        .try_fold(0_usize, |sum, transaction| {
            sum.checked_add(transaction.len())
        })
        .context("SYNC_CHECKPOINT_RESERVATION_OVERFLOW")?;
    let terms = [
        (protobuf, 16),
        (transactions, 16),
        (native.transactions.len(), 128),
        (native.frame.commit.signatures.len(), 256),
        (metadata_bytes, 4),
    ];
    terms
        .into_iter()
        .try_fold(65_536_usize, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .context("SYNC_CHECKPOINT_RESERVATION_OVERFLOW")
}

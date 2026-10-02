// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::HistoryError;
use crate::{consensus::certificates::hash_transaction_data, wire::tendermint::types::Header};

/// Reuse the exact native Data.Hash and existing development byte/count bounds.
pub(super) fn verify_block_transaction_data(
    header: &Header,
    transactions: &[Vec<u8>],
) -> Result<(), HistoryError> {
    let hash = hash_transaction_data(transactions).map_err(HistoryError::Certificate)?;
    if header.data_hash.as_slice() != hash {
        return Err(HistoryError::WrongTransactionData);
    }
    Ok(())
}

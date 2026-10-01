// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CertificateError, hashing::hash_byte_slices::hash_byte_slices};
use sha2::{Digest, Sha256};

/// Native Data.Hash hashes transaction IDs as RFC6962 leaves, never raw tx bytes.
/// Bounds are the immutable local-development raw/gas profile, not a full block-size check.
pub fn hash_transaction_data<T: AsRef<[u8]>>(
    transactions: &[T],
) -> Result<[u8; 32], CertificateError> {
    if transactions.len() > 30_000_000 / 21_000 {
        return Err(CertificateError::InvalidTransactionData);
    }
    let mut total = 0_usize;
    for transaction in transactions {
        let size = transaction.as_ref().len();
        if size > 131_072 {
            return Err(CertificateError::InvalidTransactionData);
        }
        total = total
            .checked_add(size)
            .ok_or(CertificateError::InvalidTransactionData)?;
    }
    if total > 4_194_304 {
        return Err(CertificateError::InvalidTransactionData);
    }
    let leaves: Vec<_> = transactions
        .iter()
        .map(|transaction| Sha256::digest(transaction.as_ref()).to_vec())
        .collect();
    Ok(hash_byte_slices(&leaves))
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::LocatedTransaction;
use crate::support::native_decoding::NativeBlock;
use alloy_primitives::Bytes;
use anyhow::{Result, ensure};
use sha2::{Digest, Sha256};

pub(super) fn locate_submitted_transaction(
    block: NativeBlock,
    bytes: &Bytes,
    expected_hash: &[u8; 32],
) -> Result<Option<LocatedTransaction>> {
    let index = {
        let mut matches = block
            .transactions
            .iter()
            .enumerate()
            .filter(|(_, raw)| raw.as_slice() == bytes.as_ref());
        let Some((index, raw)) = matches.next() else {
            return Ok(None);
        };
        ensure!(
            matches.next().is_none(),
            "B3_SUBMIT_OBSERVATION_DUPLICATE_TX"
        );
        ensure!(
            Sha256::digest(raw).as_slice() == expected_hash,
            "B3_SUBMIT_HASH_MISMATCH"
        );
        index
    };
    Ok(Some(LocatedTransaction { block, index }))
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};

pub(super) fn transaction_wire_size(transaction: &[u8]) -> Result<usize> {
    ensure!(
        transaction.len() <= 131_072,
        "native transaction byte limit"
    );
    let width = prost::encoding::encoded_len_varint(u64::try_from(transaction.len())?);
    transaction
        .len()
        .checked_add(width)
        .and_then(|size| size.checked_add(1))
        .ok_or_else(|| anyhow::anyhow!("native transaction wire size overflow"))
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use anyhow::{Result, ensure};

pub(super) fn receipt_cumulative_gas(receipt: &[u8]) -> Result<u64> {
    let mut remaining = receipt;
    let decoded = ReceiptEnvelope::decode_2718(&mut remaining)?;
    ensure!(remaining.is_empty(), "retained receipt trailing bytes");
    Ok(decoded.cumulative_gas_used())
}

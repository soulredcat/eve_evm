// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::fixture::TRANSITION_ADDRESS;
use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::Address;
use anyhow::{Result, ensure};

/// A native transition requires its actual successful canonical fixture receipt.
pub(crate) fn verify_transition_receipt(encoded: &[u8], fixture_digest: [u8; 32]) -> Result<()> {
    let mut remaining = encoded;
    let receipt = ReceiptEnvelope::decode_2718(&mut remaining)
        .map_err(|_| anyhow::anyhow!("B3_TC07_RECEIPT"))?;
    let address = Address::from_slice(&hex::decode(TRANSITION_ADDRESS)?);
    ensure!(
        receipt.is_success()
            && remaining.is_empty()
            && receipt.logs().iter().any(|log| log.address == address
                && log
                    .topics()
                    .get(1)
                    .is_some_and(|topic| topic.as_slice() == fixture_digest)),
        "B3_TC07_RECEIPT: native update source has no successful canonical fixture receipt"
    );
    Ok(())
}

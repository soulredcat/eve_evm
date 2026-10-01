// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::B256;
use anyhow::{Result, ensure};

pub(crate) fn decode_head_marker(bytes: &[u8]) -> Result<(u64, B256)> {
    ensure!(bytes.len() == 40, "malformed full-state durable marker");
    let height = u64::from_be_bytes(bytes[..8].try_into()?);
    Ok((height, B256::from_slice(&bytes[8..])))
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{parse_fixed, parse_quantity};
use alloy_primitives::U256;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
/// EIP1186 accepts a32-byte storage DATA word; minimal quantities are also supported.
pub(crate) fn parse_storage_slot(value: &Value) -> Result<U256, ErrorObjectOwned> {
    if value.as_str().is_some_and(|text| text.len() == 66) {
        return parse_fixed::<32>(value).map(U256::from_be_bytes);
    }
    parse_quantity(value)
}

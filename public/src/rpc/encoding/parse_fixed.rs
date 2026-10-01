// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::errors::rpc_error;
use super::parse_data;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn parse_fixed<const N: usize>(value: &Value) -> Result<[u8; N], ErrorObjectOwned> {
    parse_data(value, N)?
        .as_ref()
        .try_into()
        .map_err(|_| rpc_error(-32602, "incorrect fixed data length"))
}

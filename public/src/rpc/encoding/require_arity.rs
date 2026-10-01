// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::errors::rpc_error;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn require_arity(
    params: &[Value],
    minimum: usize,
    maximum: usize,
) -> Result<(), ErrorObjectOwned> {
    if params.len() < minimum || params.len() > maximum {
        return Err(rpc_error(-32602, "incorrect parameter count"));
    }
    Ok(())
}

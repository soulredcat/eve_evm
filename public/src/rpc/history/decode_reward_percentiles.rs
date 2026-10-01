// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::errors::rpc_error;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn decode_reward_percentiles(value: &Value) -> Result<Vec<f64>, ErrorObjectOwned> {
    let values = value
        .as_array()
        .ok_or_else(|| rpc_error(-32602, "reward percentiles must be array"))?;
    if values.len() > 100 {
        return Err(rpc_error(-32602, "reward percentile count cap100 exceeded"));
    }
    let mut percentiles = Vec::new();
    let mut previous = 0.0;
    for value in values {
        let percentile = value
            .as_f64()
            .ok_or_else(|| rpc_error(-32602, "invalid percentile"))?;
        if !(0.0..=100.0).contains(&percentile) || percentile < previous {
            return Err(rpc_error(-32602, "percentiles must be ordered0..100"));
        }
        previous = percentile;
        percentiles.push(percentile);
    }
    Ok(percentiles)
}

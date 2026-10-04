// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{NativeRpcConfig, request_native_json};
use anyhow::{Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use eve_state::{
    MAXIMUM_STATE_DELTA_CHUNK_BYTES, StateDeltaChunk, StateDeltaRequest, decode_state_delta_chunk,
    encode_state_delta_request,
};

pub(super) fn fetch_state_delta_chunk(
    config: NativeRpcConfig,
    request: &StateDeltaRequest,
    reserved_bytes: usize,
) -> Result<StateDeltaChunk> {
    let request_bytes = encode_state_delta_request(request)
        .map_err(|error| anyhow::anyhow!("SYNC_DELTA_REQUEST: {error:?}"))?;
    let path = format!(
        "/abci_query?path=%22/eve/recovery/v1/delta%22&data=0x{}&height=0&prove=false",
        hex::encode(request_bytes)
    );
    let response = request_native_json(config, &path, reserved_bytes)?;
    let response = &response["response"];
    let code = response["code"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("SYNC_DELTA_STATUS"))?;
    ensure!(
        code == 0,
        "SYNC_DELTA_REMOTE_REFUSAL: {}",
        response["codespace"].as_str().unwrap_or("UNKNOWN")
    );
    let text = response["value"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("SYNC_DELTA_VALUE"))?;
    let maximum = MAXIMUM_STATE_DELTA_CHUNK_BYTES + 16_384;
    ensure!(
        text.len() <= maximum.div_ceil(3) * 4,
        "SYNC_DELTA_ENCODED_LIMIT"
    );
    let bytes = STANDARD
        .decode(text)
        .map_err(|_| anyhow::anyhow!("SYNC_DELTA_BASE64"))?;
    ensure!(bytes.len() <= maximum, "SYNC_DELTA_CHUNK_LIMIT");
    let chunk = decode_state_delta_chunk(&bytes)
        .map_err(|error| anyhow::anyhow!("SYNC_DELTA_CHUNK: {error:?}"))?;
    #[cfg(test)]
    super::tests::hold_materialization();
    Ok(chunk)
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Serialize;

#[derive(Serialize)]
pub(super) struct NativeGenesisDocument {
    pub genesis_time: String,
    pub chain_id: String,
    pub initial_height: String,
    pub consensus_params: NativeGenesisParameters,
    pub validators: Vec<NativeGenesisValidator>,
    pub app_hash: String,
    pub app_state: serde_json::Value,
}

#[derive(Serialize)]
pub(super) struct NativeGenesisValidator {
    pub address: String,
    pub pub_key: NativeGenesisPublicKey,
    pub power: String,
    pub name: String,
}

#[derive(Serialize)]
pub(super) struct NativeGenesisPublicKey {
    #[serde(rename = "type")]
    pub key_type: String,
    pub value: String,
}

#[derive(Serialize)]
pub(super) struct NativeGenesisParameters {
    pub block: NativeBlockParameters,
    pub evidence: NativeEvidenceParameters,
    pub validator: NativeValidatorParameters,
    pub version: NativeVersionParameters,
    pub abci: NativeAbciParameters,
    pub authority: NativeAuthorityParameters,
}
#[derive(Serialize)]
pub(super) struct NativeBlockParameters {
    pub max_bytes: String,
    pub max_gas: String,
}
#[derive(Serialize)]
pub(super) struct NativeEvidenceParameters {
    pub max_age_num_blocks: String,
    pub max_age_duration: String,
    pub max_bytes: String,
}
#[derive(Serialize)]
pub(super) struct NativeValidatorParameters {
    pub pub_key_types: Vec<String>,
}
#[derive(Serialize)]
pub(super) struct NativeVersionParameters {
    pub app: String,
}
#[derive(Serialize)]
pub(super) struct NativeAbciParameters {
    pub vote_extensions_enable_height: String,
}
#[derive(Serialize)]
pub(super) struct NativeAuthorityParameters {
    pub authority: String,
}

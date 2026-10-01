// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_base64::decode_base64, decode_decimal::decode_decimal, decode_hex::decode_hex};
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::certificates::{ClassicalValidator, validator_address};
use serde_json::Value;
pub(crate) fn decode_native_validators(value: &Value) -> Result<Vec<ClassicalValidator>> {
    let values = value
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("native validator array missing"))?;
    ensure!(
        !values.is_empty() && values.len() <= super::types::MAXIMUM_DECODED_VALIDATORS,
        "native validator count bound"
    );
    let mut addresses = std::collections::BTreeSet::new();
    values
        .iter()
        .map(|value| {
            ensure!(
                value["pub_key"]["type"].as_str() == Some("tendermint/PubKeyEd25519"),
                "unsupported native validator key type"
            );
            let public_key: [u8; 32] = decode_base64(&value["pub_key"]["value"], 32, false)?
                .try_into()
                .map_err(|_| anyhow::anyhow!("native public key width"))?;
            let address = decode_hex(&value["address"], 20, false)?;
            ensure!(
                address == validator_address(&public_key) && addresses.insert(address),
                "native validator address binding/duplicate"
            );
            let voting_power = decode_decimal(&value["voting_power"])?;
            ensure!(voting_power > 0, "native voting power range");
            Ok(ClassicalValidator {
                public_key,
                voting_power,
            })
        })
        .collect()
}

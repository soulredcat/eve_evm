// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_base64::decode_base64, decode_decimal::decode_decimal, decode_hex::decode_hex};
use crate::consensus::certificates::{ClassicalValidator, validator_address};
use anyhow::{Result, ensure};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn decode_native_validator(
    value: &Value,
    addresses: &mut BTreeSet<Vec<u8>>,
) -> Result<ClassicalValidator> {
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
}

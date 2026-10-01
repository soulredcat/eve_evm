// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Cluster;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::verify_native_ed25519_signature;
use eve_storage::records::{
    OpaqueRecordIdentity, development_opaque_record_budget, opaque_record_cursor,
    open_opaque_record_repository, read_opaque_record,
};
use sha2::{Digest, Sha256};

pub(crate) fn read_signing_history(
    cluster: &Cluster,
    node: usize,
) -> Result<Vec<serde_json::Value>> {
    ensure!(
        cluster.nodes[node].child.is_none(),
        "stop signer before inspecting durable records"
    );
    let identity = OpaqueRecordIdentity {
        genesis_hash: cluster.genesis.target.identity.genesis.0.0,
        owner: cluster.nodes[node].public_key,
        domain: Sha256::digest(b"EVE_VALIDATOR_SIGNING_HISTORY_V1").into(),
    };
    let repository = open_opaque_record_repository(
        &cluster.nodes[node].data.join("signer"),
        identity,
        development_opaque_record_budget(),
    )?;
    let head = opaque_record_cursor(&repository)?;
    let mut records = Vec::new();
    let mut last = None;
    for sequence in 1..=head.sequence {
        let stored = read_opaque_record(&repository, sequence)?.context("signer record missing")?;
        let value: serde_json::Value = serde_json::from_slice(&stored.payload)?;
        let hrs = (
            value["hrs"]["height"].as_i64().context("record H")?,
            value["hrs"]["round"].as_i64().context("record R")?,
            value["hrs"]["step"].as_u64().context("record S")?,
        );
        ensure!(
            last.is_none_or(|prior| prior < hrs),
            "durable signing history repeated/conflicted HRS"
        );
        let bytes: Vec<u8> = serde_json::from_value(value["sign_bytes"].clone())?;
        let signature: Vec<u8> = serde_json::from_value(value["signature"].clone())?;
        verify_native_ed25519_signature(&cluster.nodes[node].public_key, &bytes, &signature)
            .map_err(|error| {
                anyhow::anyhow!("actual stored native signature rejected: {error:?}")
            })?;
        last = Some(hrs);
        records.push(value);
    }
    Ok(records)
}

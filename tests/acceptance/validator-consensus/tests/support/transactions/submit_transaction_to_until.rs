// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_observed_height::decode_observed_height,
    observe_submitted_transaction::observe_submitted_transaction,
    submission_rpc_until::submission_rpc_until,
    validate_admitted_submission::validate_admitted_submission,
};
use alloy_primitives::Bytes;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::{net::SocketAddr, time::Instant};

pub(super) fn submit_transaction_to_until(
    address: SocketAddr,
    bytes: &Bytes,
    deadline: Instant,
) -> Result<i64> {
    let status = submission_rpc_until(address, "/status", deadline)?;
    let first_height = decode_observed_height(&status["sync_info"]["latest_block_height"])?
        .checked_add(1)
        .context("B3_SUBMIT_OBSERVATION_HEIGHT_OVERFLOW")?;
    let expected_hash = Sha256::digest(bytes).into();
    // Admission is submitted exactly once; an ambiguous response cannot authorize resubmission.
    let admitted = submission_rpc_until(
        address,
        &format!("/broadcast_tx_sync?tx=0x{}", hex::encode(bytes)),
        deadline,
    )?;
    validate_admitted_submission(&admitted, &expected_hash)?;
    observe_submitted_transaction(address, bytes, &expected_hash, first_height, deadline)
}

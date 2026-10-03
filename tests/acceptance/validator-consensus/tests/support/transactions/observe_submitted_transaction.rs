// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    decode_observed_height::decode_observed_height,
    ensure_submission_deadline::ensure_submission_deadline,
    locate_submitted_transaction::locate_submitted_transaction,
    submission_rpc_until::submission_rpc_until,
    validate_observed_execution::validate_observed_execution,
    wait_for_submission_poll::wait_for_submission_poll,
};
use crate::support::native_decoding::decode_native_block;
use alloy_primitives::Bytes;
use anyhow::{Context, Result, ensure};
use std::{net::SocketAddr, time::Instant};

pub(super) fn observe_submitted_transaction(
    address: SocketAddr,
    bytes: &Bytes,
    expected_hash: &[u8; 32],
    mut next_height: i64,
    deadline: Instant,
) -> Result<i64> {
    loop {
        let status = submission_rpc_until(address, "/status", deadline)?;
        let native = decode_observed_height(&status["sync_info"]["latest_block_height"])?;
        let info = submission_rpc_until(address, "/abci_info", deadline)?;
        let application = decode_observed_height(&info["response"]["last_block_height"])?;
        while next_height <= native.min(application) {
            let result =
                submission_rpc_until(address, &format!("/block?height={next_height}"), deadline)?;
            let block = decode_native_block(&result).context("B3_SUBMIT_OBSERVATION_BLOCK")?;
            ensure!(
                block.header.height == next_height,
                "B3_SUBMIT_OBSERVATION_BLOCK_HEIGHT"
            );
            if let Some(located) = locate_submitted_transaction(block, bytes, expected_hash)? {
                let result = submission_rpc_until(
                    address,
                    &format!("/block_results?height={next_height}"),
                    deadline,
                )?;
                let height = validate_observed_execution(&result, &located)?;
                ensure_submission_deadline(deadline)?;
                return Ok(height);
            }
            next_height = next_height
                .checked_add(1)
                .context("B3_SUBMIT_OBSERVATION_HEIGHT_OVERFLOW")?;
        }
        wait_for_submission_poll(deadline)?;
    }
}

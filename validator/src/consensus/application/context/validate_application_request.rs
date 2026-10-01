// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::application::ConsensusApplication;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::hash_transaction_data;
use eve_storage::state::read_state_service;
use prost_types::Timestamp;

pub(in crate::consensus::application) fn validate_application_request(
    application: &ConsensusApplication,
    height: i64,
    time: Option<&Timestamp>,
    transactions: &[Vec<u8>],
) -> Result<()> {
    ensure!(!application.fenced, "application fenced");
    let parent = read_state_service(&application.service)?;
    ensure!(
        height > 0
            && parent
                .commit()
                .target
                .height
                .checked_add(1)
                .and_then(|height| i64::try_from(height).ok())
                == Some(height),
        "application request height differs from actual parent"
    );
    let time = time.context("application timestamp missing")?;
    ensure!(
        time.seconds >= 0
            && (0..1_000_000_000).contains(&time.nanos)
            && u64::try_from(time.seconds)? >= parent.commit().target.timestamp,
        "invalid application consensus timestamp"
    );
    hash_transaction_data(transactions)
        .map_err(|_| anyhow::anyhow!("native proposal transaction envelope limits"))?;
    Ok(())
}

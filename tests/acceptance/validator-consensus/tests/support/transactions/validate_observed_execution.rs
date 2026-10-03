// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    types::{LocatedTransaction, SubmissionStage},
    validate_submission_code::validate_submission_code,
    validate_submission_height::validate_submission_height,
};
use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn validate_observed_execution(
    result: &Value,
    located: &LocatedTransaction,
) -> Result<i64> {
    let height = validate_submission_height(result)?;
    ensure!(
        height == located.block.header.height,
        "B3_SUBMIT_BLOCK_RESULTS_HEIGHT"
    );
    let results = result
        .get("txs_results")
        .and_then(Value::as_array)
        .context("B3_SUBMIT_BLOCK_RESULTS_MISSING")?;
    ensure!(
        results.len() == located.block.transactions.len(),
        "B3_SUBMIT_BLOCK_RESULTS_COUNT"
    );
    let execution = results
        .get(located.index)
        .context("B3_SUBMIT_BLOCK_RESULTS_INDEX")?;
    validate_submission_code(execution, SubmissionStage::Execution)?;
    Ok(height)
}

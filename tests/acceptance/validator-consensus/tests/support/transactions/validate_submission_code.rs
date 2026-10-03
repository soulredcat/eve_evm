// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::SubmissionStage;
use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn validate_submission_code(result: &Value, stage: SubmissionStage) -> Result<()> {
    let (field, missing, invalid, rejected) = match stage {
        SubmissionStage::CheckTx => (
            "check_tx",
            "B3_SUBMIT_CHECK_TX_MISSING",
            "B3_SUBMIT_CHECK_TX_CODE",
            "B3_SUBMIT_CHECK_TX_REJECTED",
        ),
        SubmissionStage::Execution => (
            "tx_result",
            "B3_SUBMIT_TX_RESULT_MISSING",
            "B3_SUBMIT_TX_RESULT_CODE",
            "B3_SUBMIT_TX_RESULT_REJECTED",
        ),
    };
    let response = result
        .get(field)
        .and_then(Value::as_object)
        .context(missing)?;
    let code = response
        .get("code")
        .and_then(Value::as_u64)
        .filter(|code| *code <= u64::from(u32::MAX))
        .context(invalid)?;
    ensure!(code == 0, "{rejected}");
    Ok(())
}

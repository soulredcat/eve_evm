// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    types::SubmissionStage, validate_submission_code::validate_submission_code,
    validate_submission_hash::validate_submission_hash,
    validate_submission_height::validate_submission_height,
};
use anyhow::Result;
use serde_json::Value;

pub(super) fn decode_committed_submission(result: &Value, expected_hash: &[u8; 32]) -> Result<i64> {
    validate_submission_code(result, SubmissionStage::CheckTx)?;
    validate_submission_code(result, SubmissionStage::Execution)?;
    validate_submission_hash(result, expected_hash)?;
    validate_submission_height(result)
}

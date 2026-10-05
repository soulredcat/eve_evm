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

/// Compose the shared admission and execution decoders over one combined response.
///
/// Scenarios submit through admission and observation; this keeps the shared
/// code, hash and height validators covered against one canonical result shape.
pub(super) fn decode_committed_submission(result: &Value, expected_hash: &[u8; 32]) -> Result<i64> {
    validate_submission_code(&result["check_tx"], SubmissionStage::CheckTx)?;
    validate_submission_code(&result["tx_result"], SubmissionStage::Execution)?;
    validate_submission_hash(result, expected_hash)?;
    validate_submission_height(result)
}

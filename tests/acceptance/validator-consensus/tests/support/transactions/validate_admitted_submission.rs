// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    types::SubmissionStage, validate_submission_code::validate_submission_code,
    validate_submission_hash::validate_submission_hash,
};
use anyhow::Result;
use serde_json::Value;

pub(super) fn validate_admitted_submission(result: &Value, expected_hash: &[u8; 32]) -> Result<()> {
    validate_submission_code(result, SubmissionStage::CheckTx)?;
    validate_submission_hash(result, expected_hash)
}

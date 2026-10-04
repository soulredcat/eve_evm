// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointBody, CheckpointError};
use eve_state::StateCommitPreflight;

pub fn preflight_checkpoint_body(
    body: &CheckpointBody,
) -> Result<StateCommitPreflight<'_>, CheckpointError> {
    eve_state::preflight_state_commit(&body.bytes, &body.budget).map_err(CheckpointError::State)
}

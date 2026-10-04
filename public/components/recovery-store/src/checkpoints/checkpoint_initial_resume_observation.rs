// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointResumeObservation, CheckpointTransfer};

/// Initial actual-disk observation, captured at begin; later writes require fresh chunk observation.
pub fn checkpoint_initial_resume_observation(
    transfer: &CheckpointTransfer,
) -> CheckpointResumeObservation {
    transfer.initial_resume
}

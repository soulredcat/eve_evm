// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointProofTransfer;
use crate::checkpoints::CheckpointResumeObservation;

pub fn checkpoint_proof_initial_resume_observation(
    transfer: &CheckpointProofTransfer,
) -> CheckpointResumeObservation {
    transfer.initial_resume
}

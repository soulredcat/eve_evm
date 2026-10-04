// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointProofKind, CheckpointProofReferenceInput, CheckpointProofTransfer,
    types::{HEADER_BYTES, REFERENCE_BYTES},
};
use crate::checkpoints::CheckpointError;

pub(super) fn read_checkpoint_proof_reference(
    transfer: &CheckpointProofTransfer,
    index: usize,
) -> Result<CheckpointProofReferenceInput, CheckpointError> {
    if index >= transfer.summary.files {
        return Err(CheckpointError::InvalidManifest);
    }
    let position = HEADER_BYTES + transfer.summary.version_bytes + index * REFERENCE_BYTES;
    let reference = &transfer.manifest[position..position + REFERENCE_BYTES];
    Ok(CheckpointProofReferenceInput {
        height: u64::from_be_bytes(reference[..8].try_into().unwrap()),
        kind: if reference[8] == 0 {
            CheckpointProofKind::Execution
        } else {
            CheckpointProofKind::ClosingLookahead
        },
        length: usize::try_from(u64::from_be_bytes(reference[16..24].try_into().unwrap()))
            .map_err(|_| CheckpointError::InvalidManifest)?,
        sha256: reference[24..56].try_into().unwrap(),
    })
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointProofKind, CheckpointProofReferenceInput};
use crate::checkpoints::CheckpointError;

pub(super) fn encode_checkpoint_proof_reference(
    reference: &CheckpointProofReferenceInput,
) -> Result<[u8; 56], CheckpointError> {
    let mut bytes = [0_u8; 56];
    bytes[..8].copy_from_slice(&reference.height.to_be_bytes());
    bytes[8] = match reference.kind {
        CheckpointProofKind::Execution => 0,
        CheckpointProofKind::ClosingLookahead => 1,
    };
    bytes[16..24].copy_from_slice(
        &u64::try_from(reference.length)
            .map_err(|_| CheckpointError::InvalidManifest)?
            .to_be_bytes(),
    );
    bytes[24..].copy_from_slice(&reference.sha256);
    Ok(bytes)
}

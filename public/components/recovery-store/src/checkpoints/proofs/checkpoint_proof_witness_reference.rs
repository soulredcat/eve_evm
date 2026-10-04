// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointProofReferenceInput, CheckpointProofWitness};

pub fn checkpoint_proof_witness_reference(
    witness: &CheckpointProofWitness,
) -> CheckpointProofReferenceInput {
    witness.reference
}

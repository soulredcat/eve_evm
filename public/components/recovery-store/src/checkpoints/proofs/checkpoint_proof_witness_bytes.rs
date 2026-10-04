// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointProofWitness;

pub fn checkpoint_proof_witness_bytes(witness: &CheckpointProofWitness) -> &[u8] {
    &witness.bytes
}

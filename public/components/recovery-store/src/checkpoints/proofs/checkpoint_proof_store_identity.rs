// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Untrusted expected local identities; equality establishes no consensus or activation authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointProofStoreIdentity {
    pub manifest_id: [u8; 32],
    pub snapshot_manifest_id: [u8; 32],
    pub snapshot_body_sha256: [u8; 32],
    pub stream_sha256: [u8; 32],
}

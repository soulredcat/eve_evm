// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_storage::checkpoints::messages::{CheckpointResponse, CheckpointResponseStats};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointWitnessHeights {
    pub checkpoint: u64,
    pub witness: u64,
}

/// Bounded decoded peer data. Retained caller leases confer no consensus authority.
pub struct DownloadedCheckpointResponse<L> {
    pub(super) response: CheckpointResponse,
    pub(super) stats: CheckpointResponseStats,
    pub(super) _leases: [L; 2],
}
/// Immutable untrusted witness wire. Every component charge survives with the output.
pub struct DownloadedCheckpointWitness<L> {
    pub(super) wire: Vec<u8>,
    pub(super) _leases: CheckpointWitnessLeases<L>,
}
pub(super) struct CheckpointWitnessLeases<L> {
    pub(super) _metadata: Option<[L; 2]>,
    pub(super) _native: L,
    pub(super) _materialization: L,
}

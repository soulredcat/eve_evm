// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedWorkingReservation;
use eve_state::StateVersion;
use std::fs::File;

pub(in crate::runtime::follower) struct CheckpointDirectories {
    pub(in crate::runtime::follower) content: File,
    pub(in crate::runtime::follower) proofs: File,
}

pub(super) struct ChargedCheckpointGenesis {
    pub(super) version: StateVersion,
    pub(super) _lease: AppliedWorkingReservation,
}

pub(super) struct CheckpointManifestDownload {
    pub(super) downloaded: eve_sync_client::DownloadedCheckpointResponse<
        super::ingress_reservation_types::CheckpointIngressReservation,
    >,
    pub(super) id: [u8; 32],
    pub(super) stats: eve_storage::checkpoints::CheckpointManifestStats,
}

pub(super) struct CheckpointDownloadContext<'a> {
    pub(super) reader: &'a crate::sync::applied::AppliedReader,
    pub(super) genesis: &'a StateVersion,
    pub(super) address: std::net::SocketAddr,
    pub(super) limits: crate::sync::applied::checkpoints::AppliedCheckpointLimits,
    pub(super) message: eve_storage::checkpoints::messages::CheckpointMessageLimits,
    pub(super) deadline: std::time::Instant,
}

pub(super) struct ChargedCheckpointProofManifest {
    pub(super) bytes: Vec<u8>,
    pub(super) _lease: super::ingress_reservation_types::CheckpointIngressReservation,
}

pub(super) const CHECKPOINT_CHUNK_BYTES: usize = 32_768;
pub(in crate::runtime::follower) const CHECKPOINT_MAX_HEIGHT: u64 = 10_000;
pub(super) const CHECKPOINT_BOOTSTRAP_SECONDS: u64 = 300;

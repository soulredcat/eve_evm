// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Serialized CLASSICAL_DEV query allowance, independent of proposal/validator signing capacity.
#[derive(Clone, Copy, Debug)]
pub(in crate::consensus) struct SnapshotServingBudget {
    pub maximum_working_bytes: usize,
    pub maximum_request_bytes: usize,
    pub maximum_body_bytes: usize,
    pub maximum_manifest_bytes: usize,
    pub maximum_chunk_bytes: usize,
}
pub(super) const CHECKPOINT_QUERY_PATH: &str = "/eve/recovery/v1/checkpoint";

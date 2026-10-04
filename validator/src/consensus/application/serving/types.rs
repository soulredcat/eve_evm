// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Configured CLASSICAL_DEV query admission, independent of proposal reservations.
/// Serial application access bounds one active query; estimates are not an RSS cap.
#[derive(Clone, Copy, Debug)]
pub(in crate::consensus) struct DeltaServingBudget {
    pub maximum_working_bytes: usize,
    pub maximum_request_bytes: usize,
    pub maximum_chunk_bytes: usize,
    pub maximum_delta_bytes: usize,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::consensus::application) enum DeltaServingError {
    WrongNetwork,
    UnsupportedVersion,
    Gap,
    ResourceLimit,
    NotReady,
    MalformedRequest,
}

pub(super) const DELTA_QUERY_PATH: &str = "/eve/recovery/v1/delta";

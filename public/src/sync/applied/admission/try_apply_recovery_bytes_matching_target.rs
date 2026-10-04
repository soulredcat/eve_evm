// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedAdmission, AppliedError, AppliedOwner, types::AppliedBackend};
use eve_state::StateVersion;

/// Peer metadata is only an untrusted equality constraint, checked before queue/publication.
pub fn try_apply_recovery_bytes_matching_target(
    owner: &mut AppliedOwner,
    bytes: &[u8],
    expected: &StateVersion,
) -> Result<AppliedAdmission, AppliedError> {
    if !matches!(&owner.backend, AppliedBackend::Segmented { .. }) {
        return Err(AppliedError::WrongMode);
    }
    crate::sync::applied::segmented::try_apply_segmented_import(owner, bytes, Some(expected))
}

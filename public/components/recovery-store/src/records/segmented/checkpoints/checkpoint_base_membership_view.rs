// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointBaseMembership, CheckpointBaseView, types::HEADER_BYTES};

/// Borrows the same stored payload whose full equality and membership were checked.
pub fn checkpoint_base_membership_view(
    membership: &CheckpointBaseMembership,
) -> CheckpointBaseView<'_> {
    let payload = &membership.record.payload;
    CheckpointBaseView {
        metadata: membership.metadata,
        security_profile: membership.security_profile,
        target_version_bytes: &payload[HEADER_BYTES..payload.len() - 32],
    }
}

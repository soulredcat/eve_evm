// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointBaseMembership, checkpoint_base_membership_view};

/// Exact borrowed canonical bytes of the checked owned row, without state authority.
pub fn checkpoint_base_membership_target_bytes(membership: &CheckpointBaseMembership) -> &[u8] {
    checkpoint_base_membership_view(membership).target_version_bytes
}

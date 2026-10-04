// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointBaseMembership;
use crate::records::OpaqueRecord;

pub fn checkpoint_base_membership_record(membership: &CheckpointBaseMembership) -> &OpaqueRecord {
    &membership.record
}

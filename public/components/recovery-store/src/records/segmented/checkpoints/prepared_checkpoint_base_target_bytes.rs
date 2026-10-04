// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::PreparedCheckpointBaseTarget;

pub fn prepared_checkpoint_base_target_bytes(target: &PreparedCheckpointBaseTarget) -> &[u8] {
    &target.encoded
}

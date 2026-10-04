// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointBasePreflight;

pub fn checkpoint_base_bytes<'a>(preflight: &CheckpointBasePreflight<'a>) -> &'a [u8] {
    preflight.bytes
}

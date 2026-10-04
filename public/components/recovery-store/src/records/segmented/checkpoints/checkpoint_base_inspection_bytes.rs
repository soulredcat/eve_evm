// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::inspection_types::CheckpointBaseInspection;

pub fn checkpoint_base_inspection_bytes<'a>(inspection: &CheckpointBaseInspection<'a>) -> &'a [u8] {
    inspection.bytes
}

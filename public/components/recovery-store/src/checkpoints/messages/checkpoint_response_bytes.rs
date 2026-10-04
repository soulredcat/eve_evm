// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointResponsePreflight;
pub fn checkpoint_response_bytes<'a>(preflight: &CheckpointResponsePreflight<'a>) -> &'a [u8] {
    preflight.bytes
}

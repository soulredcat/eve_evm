// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitPreflight;
pub fn state_commit_preflight_bytes<'a>(preflight: &StateCommitPreflight<'a>) -> &'a [u8] {
    preflight.bytes
}

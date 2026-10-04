// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitPreflight;

/// Borrow the canonical target encoding already scanned from the sealed input.
/// Local representation equality does not authenticate this state version.
pub fn state_commit_preflight_target_bytes<'a>(preflight: &StateCommitPreflight<'a>) -> &'a [u8] {
    preflight.target_bytes
}

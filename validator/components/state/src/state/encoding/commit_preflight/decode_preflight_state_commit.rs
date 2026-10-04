// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitPreflight;
use crate::{StateCommit, StateError, decode_state_commit};
/// Only the sealed same bytes/budget are decoded; canonical decoder/reencoding retain all semantics.
pub fn decode_preflight_state_commit(
    preflight: &StateCommitPreflight<'_>,
) -> Result<StateCommit, StateError> {
    decode_state_commit(preflight.bytes, &preflight.budget)
}

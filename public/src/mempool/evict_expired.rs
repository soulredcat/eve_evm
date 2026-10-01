// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::PoolState;
use std::time::Instant;
pub(crate) fn evict_expired(state: &mut PoolState, now: Instant) -> usize {
    let mut expired = Vec::new();
    for (sender, entries) in &state.entries {
        for (nonce, entry) in entries {
            if now.saturating_duration_since(entry.admitted_at) >= state.limits.ttl {
                expired.push((*sender, *nonce));
            }
        }
    }
    let count = expired.len();
    for (sender, nonce) in expired {
        if let Some(entry) = state
            .entries
            .get_mut(&sender)
            .and_then(|entries| entries.remove(&nonce))
        {
            state.bytes -= entry.raw.len();
            state.hashes.remove(&entry.admission.hash);
        }
    }
    state.entries.retain(|_, entries| !entries.is_empty());
    count
}

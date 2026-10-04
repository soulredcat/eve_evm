// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{persistence::handoff::recovery_payload_bytes, sync::applied::RetainedAppliedTail};

pub fn retained_tail_bytes(tail: &RetainedAppliedTail, index: usize) -> Option<&[u8]> {
    tail.pending
        .get(index)
        .and_then(|pending| match &pending.payload {
            crate::sync::applied::types::PendingPayload::Compact { payload, .. } => {
                Some(recovery_payload_bytes(payload))
            }
            crate::sync::applied::types::PendingPayload::Segmented { .. } => None,
        })
}

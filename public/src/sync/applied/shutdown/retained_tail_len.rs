// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::RetainedAppliedTail;

pub fn retained_tail_len(tail: &RetainedAppliedTail) -> usize {
    tail.pending.len()
}

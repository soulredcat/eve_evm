// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::RetainedAppliedTail;

/// A retained candidate/base tail remains unresolved; it is never reported as activated.
pub fn retained_checkpoint_activation(tail: &RetainedAppliedTail) -> bool {
    tail.checkpoint.is_some() || tail.checkpoint_tail.is_some()
}

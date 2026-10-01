// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ApprovalRegistry, RegistryState};
use std::sync::{Arc, Mutex};

pub(in crate::consensus) fn create_approval_registry() -> Arc<ApprovalRegistry> {
    Arc::new(ApprovalRegistry {
        state: Mutex::new(RegistryState::default()),
    })
}

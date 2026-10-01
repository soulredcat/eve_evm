// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateService;
use crate::state::StateReader;
use std::sync::RwLock;

pub fn create_state_service(reader: StateReader) -> StateService {
    StateService {
        reader,
        cache: RwLock::new(None),
    }
}

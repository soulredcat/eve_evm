// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::engine::OwnedEngine;
use std::ops::Drop;

impl Drop for OwnedEngine {
    fn drop(&mut self) {
        super::finish_owned_engine::finish_owned_engine(self);
    }
}

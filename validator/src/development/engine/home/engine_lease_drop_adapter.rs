// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::ops::Drop;

impl Drop for super::EngineLease {
    fn drop(&mut self) {
        super::finish_engine_lease::finish_engine_lease(self);
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::ApplicationListener;
use std::ops::Drop;

impl Drop for ApplicationListener {
    fn drop(&mut self) {
        super::finish_application_listener::finish_application_listener(self);
    }
}

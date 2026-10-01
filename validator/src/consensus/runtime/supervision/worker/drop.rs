// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::types::ChannelWorkerGuard;
use std::ops::Drop;

impl Drop for ChannelWorkerGuard<'_> {
    fn drop(&mut self) {
        super::handle_channel_worker_drop::handle_channel_worker_drop(self.node);
    }
}

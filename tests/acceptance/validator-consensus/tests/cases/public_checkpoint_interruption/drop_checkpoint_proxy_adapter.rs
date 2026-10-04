// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{finish_checkpoint_proxy::finish_checkpoint_proxy, types::CheckpointProxy};

impl Drop for CheckpointProxy {
    fn drop(&mut self) {
        let _ = finish_checkpoint_proxy(self);
    }
}

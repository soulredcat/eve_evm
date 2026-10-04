// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{release_checkpoint_read::release_checkpoint_read, types::CheckpointReadLease};

impl Drop for CheckpointReadLease<'_> {
    fn drop(&mut self) {
        release_checkpoint_read(self);
    }
}

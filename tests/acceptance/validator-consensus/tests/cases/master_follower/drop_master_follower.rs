// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{stop_master_follower::stop_master_follower, types::MasterFollower};

impl std::ops::Drop for MasterFollower {
    fn drop(&mut self) {
        if stop_master_follower(self).is_err() {
            // Only the child handle created by this fixture is eligible for cleanup.
            if let Some(child) = self.child.as_mut() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

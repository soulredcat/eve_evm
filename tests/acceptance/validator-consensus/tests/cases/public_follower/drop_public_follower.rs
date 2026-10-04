// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{stop_public_follower::stop_public_follower, types::PublicFollower};

impl std::ops::Drop for PublicFollower {
    fn drop(&mut self) {
        if stop_public_follower(self).is_err() {
            // Last-resort cleanup acts only on the child handle created by this fixture.
            if let Some(child) = self.child.as_mut() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::types::WorkerLifetimeLease, release_segmented_worker::release_segmented_worker,
};

impl std::ops::Drop for WorkerLifetimeLease {
    fn drop(&mut self) {
        release_segmented_worker(self);
    }
}

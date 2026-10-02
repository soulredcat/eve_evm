// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{release_handoff_reservation::release_handoff_reservation, types::ReservationLease};

impl std::ops::Drop for ReservationLease {
    fn drop(&mut self) {
        release_handoff_reservation(self);
    }
}

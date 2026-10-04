// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod bind_segmented_reservation;
mod reserve_segmented_batch;
mod reserve_segmented_layout;
pub use bind_segmented_reservation::bind_segmented_reservation;
pub use reserve_segmented_batch::reserve_segmented_batch;
pub use reserve_segmented_layout::reserve_segmented_layout;

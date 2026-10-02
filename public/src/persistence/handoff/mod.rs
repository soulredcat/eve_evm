// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod borrow_recovery_record;
mod create_handoff_pool;
mod drop_reservation_adapter;
mod handoff_budget;
mod observe_handoff;
mod payload_belongs_to_pool;
mod payload_reservation_id;
mod recovery_payload_bytes;
mod release_handoff_reservation;
mod reserve_recovery_payload;
mod seal_recovery_payload;
mod types;
mod write_reserved_payload;

pub(in crate::persistence) use borrow_recovery_record::borrow_recovery_record;
pub use create_handoff_pool::create_handoff_pool;
pub(in crate::persistence) use handoff_budget::handoff_budget;
pub use observe_handoff::observe_handoff;
pub(in crate::persistence) use payload_belongs_to_pool::payload_belongs_to_pool;
pub(in crate::persistence) use payload_reservation_id::payload_reservation_id;
pub use recovery_payload_bytes::recovery_payload_bytes;
pub use reserve_recovery_payload::reserve_recovery_payload;
pub use seal_recovery_payload::seal_recovery_payload;
pub use types::{
    HandoffError, HandoffObservation, HandoffPool, PayloadReservation, RecoveryPayload,
};
pub use write_reserved_payload::write_reserved_payload;

#[cfg(test)]
mod tests;

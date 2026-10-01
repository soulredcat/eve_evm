// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod ack;
mod budget;
mod cursor;
mod identity;
mod record;
mod repository;
pub(super) mod schema;

pub use ack::{OpaqueRecordAck, OpaqueRecordDisposition};
pub use budget::OpaqueRecordBudget;
pub use cursor::OpaqueRecordCursor;
pub use identity::OpaqueRecordIdentity;
pub use record::OpaqueRecord;
pub use repository::OpaqueRecordRepository;
#[cfg(test)]
pub(super) use repository::SimulatedOpaqueFailure;

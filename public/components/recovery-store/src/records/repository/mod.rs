// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(super) mod append;
mod opening;
mod reading;
pub use append::compare_and_append_opaque_records;
pub use opening::open_opaque_record_repository;
pub use reading::{opaque_record_cursor, read_opaque_record};

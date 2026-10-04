// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedOwner, AppliedReader};

/// Capture the reader of this exact admission owner and its actual working pool.
pub fn applied_owner_reader(owner: &AppliedOwner) -> AppliedReader {
    owner.reader.clone()
}

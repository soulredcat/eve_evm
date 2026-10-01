// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::engine::OwnedEngine;

pub(crate) fn engine_pid(engine: &OwnedEngine) -> Option<u32> {
    engine.child.as_ref().map(std::process::Child::id)
}

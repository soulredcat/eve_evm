// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Best-effort destructor cleanup; supervisors use stop_engine to receive failures explicitly.
pub(super) fn finish_owned_engine(engine: &mut crate::development::engine::OwnedEngine) {
    let _ = super::stop_engine(engine);
}

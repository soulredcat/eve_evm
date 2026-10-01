// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Destructor cleanup cannot return an error; explicit stop paths report unlock failure.
pub(super) fn finish_engine_lease(lease: &mut super::EngineLease) {
    let _ = super::release_engine_lease(lease);
}

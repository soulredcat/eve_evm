// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
pub(crate) fn acceptance_key_enrolled(fixture: Option<&AcceptanceFixture>, key: &[u8; 32]) -> bool {
    fixture.is_some_and(|fixture| fixture.future.contains_key(key))
}

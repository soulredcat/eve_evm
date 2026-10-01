// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
pub(crate) fn acceptance_key_enrolled(
    _fixture: Option<&AcceptanceFixture>,
    _key: &[u8; 32],
) -> bool {
    false
}

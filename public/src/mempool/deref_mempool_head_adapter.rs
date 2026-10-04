// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MempoolHead;
use eve_state::StateCommit;

/// Thin borrow-only standard trait delegation; no authority conversion or clone.
impl std::ops::Deref for MempoolHead {
    type Target = StateCommit;

    fn deref(&self) -> &Self::Target {
        self.commit()
    }
}

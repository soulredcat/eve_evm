// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MempoolHead;
use crate::sync::applied::applied_commit;
use eve_state::StateCommit;

impl MempoolHead {
    /// Only a borrow. Applied publication/generation ownership stays in this head.
    pub fn commit(&self) -> &StateCommit {
        match self {
            Self::Local(commit) => commit.as_ref(),
            Self::Applied(publication) => applied_commit(publication),
        }
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{BlockPayload, StateVersion};

/// Borrowed common encoding inputs; the caller retains the real captured state owner.
pub(crate) struct RpcBlockView<'a> {
    pub version: &'a StateVersion,
    pub block: &'a BlockPayload,
}

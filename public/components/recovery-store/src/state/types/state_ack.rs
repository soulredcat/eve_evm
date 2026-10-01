// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::B256;
use eve_state::StateVersion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommitDisposition {
    NewlySynced,
    ExactReplay,
}

/// Local successful sync/replay result; no consensus/authenticated watermark is implied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableStateAck {
    pub committed: StateVersion,
    pub store_head: StateVersion,
    pub commit_identity: B256,
    pub database_sequence: u64,
    pub disposition: CommitDisposition,
}

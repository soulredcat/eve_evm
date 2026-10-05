// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::AuthenticatedApplicationAnchor;
use eve_consensus_comet::wire::tendermint::types::BlockId;

impl AuthenticatedApplicationAnchor {
    pub fn consensus_block_id(&self) -> &BlockId {
        &self.consensus_block_id
    }
}

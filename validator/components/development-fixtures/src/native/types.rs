// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::{
    consensus::certificates::ClassicalValidator,
    wire::tendermint::types::{BlockId, Commit, Header},
};

pub struct Frame {
    pub header: Header,
    pub id: BlockId,
    pub commit: Commit,
    pub validators: Vec<ClassicalValidator>,
}

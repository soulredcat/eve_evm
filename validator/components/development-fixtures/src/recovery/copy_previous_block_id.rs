// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native::Frame;
use eve_consensus_comet::wire::tendermint::types::BlockId;

pub(super) fn copy_previous_block_id(frame: &Frame) -> BlockId {
    frame.id.clone()
}

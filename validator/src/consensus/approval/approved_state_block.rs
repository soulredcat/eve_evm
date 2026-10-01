// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ExecutionApproval;
use eve_evm::PreparedStateBlock;

pub(in crate::consensus) fn approved_state_block(
    approval: &ExecutionApproval,
) -> &PreparedStateBlock {
    &approval.prepared
}

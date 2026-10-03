// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::native_decoding::NativeBlock;

pub(super) enum SubmissionStage {
    CheckTx,
    Execution,
}

/// Test observation location only; downstream certificate verification and replay remain required.
pub(super) struct LocatedTransaction {
    pub block: NativeBlock,
    pub index: usize,
}

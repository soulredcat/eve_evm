// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::{NativeDataFrame, NativeFrame, types::capability::FixedGenesisPolicy};
use crate::{DevelopmentFinalityVerifier, VerifiedDevelopmentHeader};
use eve_state::Bytes;

/// Shared authenticated-history inputs; these create neither replay nor import authority.
pub(in crate::recovery) struct RecoveryHistoryParent<'a> {
    pub(in crate::recovery) finality: &'a DevelopmentFinalityVerifier,
    pub(in crate::recovery) height: u64,
    pub(in crate::recovery) lookahead: Option<&'a NativeDataFrame>,
    pub(in crate::recovery) lookahead_header: Option<&'a VerifiedDevelopmentHeader>,
    pub(in crate::recovery) policy: &'a FixedGenesisPolicy,
}

pub(in crate::recovery) struct RecoveryHistoryInput<'a> {
    pub(in crate::recovery) finalized: &'a NativeFrame,
    pub(in crate::recovery) transactions: &'a [Bytes],
    pub(in crate::recovery) lookahead: &'a NativeDataFrame,
}

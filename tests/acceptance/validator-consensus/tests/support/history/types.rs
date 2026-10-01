// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::native_decoding::NativeBlock;
use eve_consensus_comet::{
    consensus::certificates::ClassicalValidator, wire::tendermint::types::Commit,
};
pub(crate) struct CertifiedBlock {
    pub block: NativeBlock,
    pub commit: Commit,
    pub validators: Vec<ClassicalValidator>,
}

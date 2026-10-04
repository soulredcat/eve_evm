// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::errors::rpc_error;
use crate::sync::applied::{AppliedPublication, applied_commit};
use alloy_primitives::{B256, keccak256};
use jsonrpsee::types::ErrorObjectOwned;

pub(super) fn locate_applied_transaction(
    publication: &AppliedPublication,
    hash: B256,
) -> Result<usize, ErrorObjectOwned> {
    applied_commit(publication)
        .block
        .transactions
        .iter()
        .position(|raw| keccak256(raw) == hash)
        .ok_or_else(|| {
            rpc_error(
                -32001,
                "GAP: transaction history is outside the captured applied view",
            )
        })
}

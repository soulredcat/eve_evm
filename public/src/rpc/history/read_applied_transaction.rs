// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::locate_applied_transaction::locate_applied_transaction;
use crate::rpc::{
    encoding::{encode_transaction, parse_fixed, require_arity},
    errors::rpc_error,
};
use crate::sync::applied::{AppliedPublication, applied_commit};
use alloy_primitives::B256;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

pub(super) fn read_applied_transaction(
    publication: &AppliedPublication,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 1, 1)?;
    let index =
        locate_applied_transaction(publication, B256::from(parse_fixed::<32>(&params[0])?))?;
    let commit = applied_commit(publication);
    encode_transaction(
        &commit.block.transactions[index],
        commit.target.identity.evm_chain_id,
        Some(&commit.block.header),
        Some(index as u64),
    )
    .map_err(|error| rpc_error(-32000, error.to_string()))
}

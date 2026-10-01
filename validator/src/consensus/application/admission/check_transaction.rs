// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    application::ConsensusApplication,
    transport::peer::{AuthenticatedEnginePeer, ensure_application_engine_peer},
};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{RequestCheckTx, ResponseCheckTx};
use eve_evm::{check_transaction_admission, decode_signed_transaction};
use eve_protocol_config::headers::derive_next_base_fee;
use eve_storage::state::read_state_service;

/// Local admission permits future nonces without reserving or mutating canonical state.
pub(in crate::consensus) fn check_transaction(
    application: &ConsensusApplication,
    peer: &AuthenticatedEnginePeer,
    request: &RequestCheckTx,
) -> Result<ResponseCheckTx> {
    ensure_application_engine_peer(peer)?;
    ensure!(!application.fenced, "application fenced");
    if !matches!(request.r#type, 0 | 1) {
        return Ok(ResponseCheckTx {
            code: 1,
            codespace: "EVE_INVALID_CHECK_TX_KIND".into(),
            ..Default::default()
        });
    }
    let head = read_state_service(&application.service)?;
    let transaction = match decode_signed_transaction(
        &request.tx,
        head.commit().target.identity.evm_chain_id,
        131_072,
    ) {
        Ok(transaction) => transaction,
        Err(_) => {
            return Ok(ResponseCheckTx {
                code: 1,
                codespace: "EVE_INVALID_ENVELOPE".into(),
                ..Default::default()
            });
        }
    };
    let base_fee = derive_next_base_fee(&head.commit().block.header)
        .map_err(|_| anyhow::anyhow!("invalid durable base-fee context"))?;
    let admission = match check_transaction_admission(
        &transaction,
        head.commit().state.accounts.get(&transaction.sender()),
        base_fee,
        head.commit().block.header.gas_limit,
    ) {
        Ok(admission) if admission.nonce >= admission.state_nonce => admission,
        _ => {
            return Ok(ResponseCheckTx {
                code: 1,
                codespace: "EVE_LOCAL_ADMISSION_REJECTED".into(),
                ..Default::default()
            });
        }
    };
    Ok(ResponseCheckTx {
        code: 0,
        gas_wanted: i64::try_from(admission.gas_limit)?,
        gas_used: i64::try_from(admission.intrinsic_gas)?,
        codespace: "EVE_LOCAL_ADMISSION".into(),
        ..Default::default()
    })
}

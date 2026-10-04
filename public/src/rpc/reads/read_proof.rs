// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    RpcContext,
    encoding::{parse_fixed, parse_storage_slot, quantity, require_arity},
    errors::rpc_error,
    selectors::capture_selected_state,
};
use alloy_primitives::Address;
use eve_state::{ProofLimits, build_account_proof, capture_state_view, estimate_proof_reservation};
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
pub(crate) fn read_proof(
    context: &RpcContext,
    params: &[Value],
) -> Result<Value, ErrorObjectOwned> {
    require_arity(params, 3, 3)?;
    let address = Address::from(parse_fixed::<20>(&params[0])?);
    let slots = params[1]
        .as_array()
        .ok_or_else(|| rpc_error(-32602, "proof slots must be an array"))?;
    if slots.len() > 256 {
        return Err(rpc_error(-32602, "proof slot cap256 exceeded"));
    }
    let slots = slots
        .iter()
        .map(parse_storage_slot)
        .collect::<Result<Vec<_>, _>>()?;
    let selected = capture_selected_state(context, &params[2])?;
    let state = selected.commit();
    let _clone_lease = super::reserve_proof_state_clone::reserve_proof_state_clone(context, state)?;
    let view = capture_state_view(
        state.state.clone(),
        state.target.clone(),
        &context.state_budget,
    )
    .map_err(|e| rpc_error(-32000, format!("invalid proof view: {e:?}")))?;
    let reservation = estimate_proof_reservation(&view, slots.len())
        .map_err(|e| rpc_error(-32000, format!("proof reservation: {e:?}")))?;
    let charge = u32::try_from(reservation.div_ceil(1024))
        .map_err(|_| rpc_error(-32005, "proof reservation overflow"))?;
    let _permit = Arc::clone(&context.bytes)
        .try_acquire_many_owned(charge)
        .map_err(|_| rpc_error(-32005, "proof byte capacity exceeded"))?;
    let limits = ProofLimits {
        maximum_requested_slots: 256,
        maximum_proof_bytes: 1_048_576,
        maximum_rebuild_bytes: reservation,
    };
    let proof = build_account_proof(&view, address, &slots, &limits, reservation)
        .map_err(|e| rpc_error(-32000, format!("proof failed: {e:?}")))?;
    let response = alloy_rpc_types_eth::EIP1186AccountProofResponse {
        address: proof.address,
        balance: proof.balance,
        code_hash: proof.code_hash,
        nonce: proof.nonce,
        storage_hash: proof.storage_root,
        account_proof: proof.account_proof,
        storage_proof: proof
            .storage_proof
            .into_iter()
            .map(|slot| alloy_rpc_types_eth::EIP1186StorageProof {
                key: slot.key.into(),
                value: slot.value,
                proof: slot.proof,
            })
            .collect(),
    };
    let mut value = serde_json::to_value(response).map_err(|e| rpc_error(-32603, e.to_string()))?;
    value["eveStateRoot"] =
        serde_json::to_value(proof.state_root.0).map_err(|e| rpc_error(-32603, e.to_string()))?;
    value["eveHeight"] = quantity(proof.height);
    value["eveVerificationMode"] =
        Value::String(crate::rpc::selectors::selected_verification_mode(&selected).into());
    if let crate::rpc::selectors::SelectedState::Applied { publication } = &selected {
        let markers = crate::sync::applied::applied_markers(publication);
        value["eveAuthenticatedHeight"] =
            if crate::sync::applied::applied_anchor(publication).is_some() {
                quantity(markers.authenticated_state.0)
            } else {
                Value::Null
            };
        value["eveFinalizedHeight"] = quantity(markers.finalized.0);
    }
    Ok(value)
}
use std::sync::Arc;

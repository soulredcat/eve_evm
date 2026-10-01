// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    ApprovalRegistry, RegistryState, RetainedInput, trim_retained_inputs,
    types::MAXIMUM_FULL_APPROVALS,
};
use crate::consensus::{
    approval::{ApprovalError, ExecutionApproval, check_approval_parent},
    signing::DurableSigner,
    transport::proposals::{NativeProposalSource, VerifiedLocalEngineProposal},
};
use prost::Message;
use std::sync::Arc;

/// Consumes fresh sealed ProcessProposal provenance. Native lifecycle establishes unlocked state.
pub(in crate::consensus) fn publish_native_approval(
    registry: &ApprovalRegistry,
    signer: &DurableSigner,
    source: VerifiedLocalEngineProposal,
    approval: Arc<ExecutionApproval>,
) -> Result<(), ApprovalError> {
    let unavailable = |reason| ApprovalError::Unavailable {
        reason,
        cause: None,
    };
    if !matches!(source.source(), NativeProposalSource::ProcessProposal) {
        return Err(unavailable(
            "finalized input is not native-unlocked replacement evidence",
        ));
    }
    check_approval_parent(&approval, signer)?;
    if source.request() != &approval.request
        || source.binding().genesis_hash != approval.config.genesis_hash
        || source.binding().chain_id != approval.config.chain_id
        || source.binding().authentication != approval.config.authentication
        || source.binding().key_epoch != approval.config.key_epoch
        || source.binding().proposer_owner != approval.prepared.commit.block.header.beneficiary
        || source.binding().previous_consensus_hash.as_slice()
            != approval.prepared.commit.block.header.mix_hash.as_slice()
    {
        return Err(unavailable(
            "native publication source differs from executed approval",
        ));
    }
    let bytes = source
        .request()
        .encoded_len()
        .checked_add(source.binding().chain_id.len())
        .and_then(|value| value.checked_add(128))
        .ok_or(unavailable("retained input byte overflow"))?;
    let mut state = registry
        .state
        .lock()
        .map_err(|_| unavailable("approval registry poisoned"))?;
    if state
        .config
        .as_ref()
        .is_some_and(|config| config != &approval.config)
    {
        return Err(unavailable("approval registry belongs to another signer"));
    }
    if state.parent.as_ref() != Some(&approval.parent)
        || state.sequence != approval.database_sequence
    {
        *state = RegistryState {
            config: Some(approval.config.clone()),
            parent: Some(approval.parent.clone()),
            sequence: approval.database_sequence,
            ..RegistryState::default()
        };
    }
    let hash = approval.consensus_hash;
    if state.full.get(&hash).is_some_and(|old| {
        old.request != approval.request
            || old.prepared.commit.target != approval.prepared.commit.target
    }) {
        return Err(unavailable(
            "same native hash has conflicting executed data",
        ));
    }
    let insertion = state
        .insertion
        .checked_add(1)
        .ok_or(unavailable("approval insertion counter overflow"))?;
    let mut raw = state.raw.clone();
    raw.insert(
        hash,
        Arc::new(RetainedInput {
            source,
            bytes,
            insertion,
        }),
    );
    trim_retained_inputs(
        &mut raw,
        &[Some(hash), state.prevote_pin, state.precommit_pin],
    )?;
    let mut full = state.full.clone();
    full.retain(|key, _| *key == hash || Some(*key) == state.prevote_pin);
    full.insert(hash, approval);
    if full.len() > MAXIMUM_FULL_APPROVALS {
        return Err(unavailable("required full approval capacity unavailable"));
    }
    state.raw = raw;
    state.full = full;
    state.current = Some(hash);
    state.insertion = insertion;
    Ok(())
}

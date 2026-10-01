// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    encode_finalize_result, finalize_fresh_block::finalize_fresh_block,
    replay_finalized_result::replay_finalized_result,
    validate_finalization_binding::validate_finalization_binding,
    validate_finalize_source::validate_finalize_source,
};
use crate::consensus::{
    application::{ConsensusApplication, safety::fence_application},
    approval::approved_state_block,
    transport::proposals::VerifiedLocalEngineProposal,
};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{RequestFinalizeBlock, ResponseFinalizeBlock};
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn finalize_block(
    application: &mut ConsensusApplication,
    source: VerifiedLocalEngineProposal,
    request: &RequestFinalizeBlock,
) -> Result<ResponseFinalizeBlock> {
    ensure!(!application.fenced, "application fenced");
    validate_finalize_source(&source, request)?;
    validate_finalization_binding(application, &source)?;
    if let Some(pending) = &application.pending {
        ensure!(
            pending.request == *request,
            "different finalization before pending commit"
        );
        let prepared = approved_state_block(&pending.approval);
        return encode_finalize_result(
            &prepared.commit.target,
            &prepared.commit.block,
            application.config.acceptance_fixture.as_deref(),
        );
    }
    let head = read_state_service(&application.service)?;
    if request.height > 0 && u64::try_from(request.height)? <= head.commit().target.height {
        let response = replay_finalized_result(application, request)?;
        application.replayed_height = Some(u64::try_from(request.height)?);
        return Ok(response);
    }
    match finalize_fresh_block(application, source, request) {
        Ok(response) => Ok(response),
        Err(error) => {
            fence_application(application);
            Err(error)
        }
    }
}

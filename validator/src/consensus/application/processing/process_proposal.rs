// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{approval_logical_charge, validate_proposal_binding::validate_proposal_binding};
use crate::consensus::{
    application::{ConsensusApplication, context::validate_application_request},
    approval::{
        ApprovalError, approval_error_diagnostic, approval_request, approved_state_block,
        create_execution_approval, find_approval, publish_native_approval,
    },
    transport::proposals::{NativeProposalSource, VerifiedLocalEngineProposal},
};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::ResponseProcessProposal;
use eve_storage::state::read_state_service;
use std::sync::Arc;

pub(in crate::consensus) fn process_proposal(
    application: &mut ConsensusApplication,
    source: VerifiedLocalEngineProposal,
) -> Result<ResponseProcessProposal> {
    ensure!(
        source.source() == NativeProposalSource::ProcessProposal,
        "finalization source is not a fresh proposal callback"
    );
    ensure!(
        !application.fenced && application.pending.is_none(),
        "application unavailable for proposal execution"
    );
    if !super::proposal_transactions_fit::proposal_transactions_fit(&source.request().txs) {
        return Ok(ResponseProcessProposal { status: 2 });
    }
    validate_application_request(
        application,
        source.request().height,
        source.request().time.as_ref(),
        &source.request().txs,
    )?;
    validate_proposal_binding(application, &source)?;
    let hash: [u8; 32] = source
        .request()
        .hash
        .as_slice()
        .try_into()
        .context("native proposal hash width")?;
    let signer = application
        .signer
        .lock()
        .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
    let parent = read_state_service(&application.service)?;
    let cached = find_approval(&application.approvals, &hash)
        .map_err(|_| anyhow::anyhow!("approval registry unavailable"))?
        .filter(|approval| {
            approval_request(approval) == source.request()
                && approved_state_block(approval).commit.parent.as_ref()
                    == Some(&parent.commit().target)
        });
    let approval = match cached {
        Some(approval) => approval,
        None => match create_execution_approval(
            &signer,
            &source,
            &application.config.logical_budget,
            application.config.reserved_clone_bytes,
        ) {
            Ok(approval) => Arc::new(approval),
            Err(ApprovalError::InvalidExecution(_)) => {
                crate::development::acceptance::record_acceptance_rejection(
                    application.config.acceptance_fixture.as_deref(),
                    source.request(),
                )?;
                return Ok(ResponseProcessProposal { status: 2 });
            }
            Err(error @ ApprovalError::Unavailable { .. }) => {
                return Err(approval_error_diagnostic(error));
            }
        },
    };
    let charge = approval_logical_charge(&approval, &application.config.logical_budget)?;
    ensure!(
        charge
            <= application.config.maximum_cached_bytes
                / application.config.maximum_cached_candidates,
        "local prepared approval byte capacity unavailable"
    );
    publish_native_approval(&application.approvals, &signer, source, approval)
        .map_err(|_| anyhow::anyhow!("native approval registry capacity/context unavailable"))?;
    Ok(ResponseProcessProposal { status: 1 })
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_finalize_result;
use crate::consensus::application::{
    ConsensusApplication, reading::retained_application_block, replay::find_replay_decision,
};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{RequestFinalizeBlock, ResponseFinalizeBlock};

pub(super) fn replay_finalized_result(
    application: &ConsensusApplication,
    request: &RequestFinalizeBlock,
) -> Result<ResponseFinalizeBlock> {
    let (decision, _) = find_replay_decision(application, u64::try_from(request.height)?)?
        .context("retained decided replay input missing")?;
    ensure!(
        decision.request == *request,
        "replayed consensus block differs from exact decided input"
    );
    let retained = retained_application_block(application, decision.target.height)?;
    ensure!(
        retained.version == decision.target && retained.commit_identity == decision.commit_identity,
        "replayed application commit differs from decided metadata"
    );
    encode_finalize_result(
        &retained.version,
        &retained.block,
        application.config.acceptance_fixture.as_deref(),
    )
}

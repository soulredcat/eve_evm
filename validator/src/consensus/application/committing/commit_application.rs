// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::commit_pending_block::commit_pending_block;
use crate::consensus::{
    application::{ConsensusApplication, safety::fence_application},
    transport::peer::{AuthenticatedEnginePeer, ensure_application_engine_peer},
};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::ResponseCommit;

pub(in crate::consensus) fn commit_application(
    application: &mut ConsensusApplication,
    peer: &AuthenticatedEnginePeer,
) -> Result<ResponseCommit> {
    ensure_application_engine_peer(peer)?;
    ensure!(!application.fenced, "application fenced");
    match commit_pending_block(application) {
        Ok(response) => Ok(response),
        Err(error) => {
            fence_application(application);
            Err(error)
        }
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    consensus::{
        application::application_info,
        approval::{ApprovalRegistryStatus, approval_registry_status},
        runtime::types::{ApprovalReadiness, NodeReadiness, RunningNode},
        signing::signer_status,
    },
    development::engine::engine_child,
};
use anyhow::Result;
use std::io::Write;

pub(in crate::consensus::runtime) fn emit_node_readiness(node: &RunningNode) -> Result<bool> {
    let progress = node
        .handshake
        .lock()
        .map_err(|_| anyhow::anyhow!("readiness lock poisoned"))?;
    if !(progress.info.is_some()
        && progress.initialized
        && progress.public_key
        && (progress.ping || progress.signing_activity))
    {
        return Ok(false);
    }
    drop(progress);
    {
        let mut engine = node
            .engine
            .lock()
            .map_err(|_| anyhow::anyhow!("engine supervisor lock poisoned"))?;
        engine_child(&mut engine)?;
    }
    let application = node
        .assembly
        .application
        .lock()
        .map_err(|_| anyhow::anyhow!("application actor lock poisoned"))?;
    let info = application_info(&application)?;
    let signer = {
        let signer = node
            .assembly
            .signer
            .lock()
            .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
        signer_status(&signer)
    };
    let approvals: ApprovalRegistryStatus = approval_registry_status(&node.assembly.registry)
        .map_err(|_| anyhow::anyhow!("approval registry readiness unavailable"))?;
    anyhow::ensure!(
        approvals.full <= 2
            && approvals.raw <= 4
            && approvals.retained_encoded_bytes <= 16 * 1_048_576,
        "approval registry exceeds reviewed development bounds"
    );
    anyhow::ensure!(
        !signer.fenced
            && signer
                .last_height
                .is_none_or(|height| height <= info.last_block_height.saturating_add(1)),
        "signer history is fenced or ahead of canonical application recovery"
    );
    let readiness = NodeReadiness {
        event: "development_validator_ready",
        initialization: &node.assembly.initialization,
        application_height: info.last_block_height,
        application_hash: hex::encode(info.last_block_app_hash),
        consensus_finality: false,
        signer_last_height: signer.last_height,
        signer_last_round: signer.last_round,
        signer_last_step: signer.last_step,
        execution_approvals: ApprovalReadiness {
            prepared_states: approvals.full,
            retained_proposals: approvals.raw,
            retained_proposal_bytes: approvals.retained_encoded_bytes,
            current_candidate: approvals.current.map(hex::encode),
            protected_prevote: approvals.prevote_pin.map(hex::encode),
            protected_precommit: approvals.precommit_pin.map(hex::encode),
        },
    };
    drop(application);
    let mut output = std::io::stdout().lock();
    serde_json::to_writer(&mut output, &readiness)?;
    output.write_all(b"\n")?;
    output.flush()?;
    Ok(true)
}

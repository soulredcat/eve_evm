// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{NodeFailureRecord, SignerPosition, classify_node_failure::classify_node_failure};
use crate::consensus::{
    application::application_info, runtime::types::RunningNode, signing::signer_status,
};

pub(in crate::consensus::runtime) fn snapshot_node_failure(
    node: &RunningNode,
    error: &anyhow::Error,
) -> NodeFailureRecord {
    let application_height = node
        .assembly
        .application
        .try_lock()
        .ok()
        .and_then(|application| {
            application_info(&application)
                .ok()
                .map(|info| info.last_block_height)
        });
    let signer_last = node.assembly.signer.try_lock().ok().and_then(|signer| {
        let status = signer_status(&signer);
        Some(SignerPosition {
            height: status.last_height?,
            round: status.last_round?,
            step: i32::from(status.last_step?),
        })
    });
    let signer_requested = node
        .signer_requested
        .try_lock()
        .ok()
        .and_then(|position| *position);
    let categories = error
        .chain()
        .take(8)
        .map(|cause| classify_node_failure(&cause.to_string()))
        .collect();
    let io_kind = error
        .downcast_ref::<std::io::Error>()
        .map(|error| format!("{:?}", error.kind()));
    NodeFailureRecord {
        version: 1,
        process_id: std::process::id(),
        categories,
        io_kind,
        application_height,
        signer_last,
        signer_requested,
    }
}

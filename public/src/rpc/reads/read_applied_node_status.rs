// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::status_types::NodeStatus;
use crate::rpc::{
    RpcContext,
    errors::rpc_error,
    selectors::{SelectedState, selected_verification_mode},
};
use crate::sync::applied::{
    AppliedReader, applied_anchor, applied_cursors, applied_markers, applied_readiness,
    applied_segmented_position, applied_storage_failed, capture_applied_state,
};
use eve_node_policy::PublicReadiness;
use jsonrpsee::types::ErrorObjectOwned;
use std::sync::atomic::Ordering;

pub(super) fn read_applied_node_status(
    context: &RpcContext,
    reader: &AppliedReader,
) -> Result<NodeStatus, ErrorObjectOwned> {
    let publication = capture_applied_state(reader)
        .map_err(|_| rpc_error(-32001, "NOT_READY: applied publication unavailable"))?;
    let markers = applied_markers(&publication);
    let cursors = applied_cursors(&publication);
    let position = applied_segmented_position(&publication);
    let readiness = applied_readiness(&publication);
    let healthy = context.healthy.load(Ordering::Acquire);
    let ready = healthy && matches!(readiness, PublicReadiness::Ready { .. });
    let reason = if !healthy {
        Some("RPC service fenced")
    } else {
        match readiness {
            PublicReadiness::Ready { .. } => None,
            PublicReadiness::NotReady(reason) => Some(reason),
        }
    };
    let authenticated_finality = applied_anchor(&publication).is_some();
    let storage_failed = applied_storage_failed(&publication);
    let selected = SelectedState::Applied { publication };
    Ok(NodeStatus {
        applied_height: markers.applied.0,
        durable_height: markers.durable_recovery.0,
        authenticated_height: authenticated_finality.then_some(markers.authenticated_state.0),
        finalized_height: Some(markers.finalized.0),
        verification_mode: selected_verification_mode(&selected),
        authenticated_finality,
        ready,
        // No independently corroborated head/connection observation exists in this source.
        peer_count: None,
        lag: None,
        database_sequence: None,
        zone_id: context.zone.0,
        readiness_reason: reason,
        durable_lag: Some(markers.applied.0.saturating_sub(markers.durable_recovery.0)),
        checkpoint_height: Some(markers.checkpoint.0),
        authenticated_snapshot_height: Some(markers.authenticated_snapshot_height),
        oldest_retained_height: Some(markers.oldest_retained_height),
        storage_failed: Some(storage_failed),
        admitted_record_sequence: Some(cursors.0.sequence),
        durable_record_sequence: Some(cursors.1.sequence),
        last_acknowledged_physical_sequence: position
            .map(|position| position.last_acknowledged_physical.sequence),
        missing_recovery_from: position.and_then(|position| position.missing_from),
    })
}

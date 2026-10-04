// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::status_types::NodeStatus;
use crate::rpc::{RpcContext, RpcStateSource, errors::rpc_error};
use eve_storage::state::read_state_service;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::atomic::Ordering;
pub(crate) fn read_node_status(context: &RpcContext) -> Result<Value, ErrorObjectOwned> {
    if let RpcStateSource::Applied { reader } = &context.source {
        return serde_json::to_value(super::read_applied_node_status::read_applied_node_status(
            context, reader,
        )?)
        .map_err(|error| rpc_error(-32603, error.to_string()));
    }
    let service = crate::rpc::durable_rpc_source(context)?.0;
    let head = read_state_service(service).map_err(|e| rpc_error(-32000, e.to_string()))?;
    let status = NodeStatus {
        applied_height: head.commit().target.height,
        durable_height: head.commit().target.height,
        authenticated_height: None,
        finalized_height: None,
        verification_mode: "LOCAL_DEV_UNAUTHENTICATED",
        authenticated_finality: false,
        ready: context.healthy.load(Ordering::Acquire),
        peer_count: Some(0),
        lag: Some(0),
        database_sequence: Some(head.sequence()),
        zone_id: context.zone.0,
        readiness_reason: None,
        durable_lag: None,
        checkpoint_height: None,
        authenticated_snapshot_height: None,
        oldest_retained_height: None,
        storage_failed: None,
        admitted_record_sequence: None,
        durable_record_sequence: None,
        last_acknowledged_physical_sequence: None,
        missing_recovery_from: None,
    };
    serde_json::to_value(status).map_err(|e| rpc_error(-32603, e.to_string()))
}

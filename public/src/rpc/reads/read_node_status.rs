// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::status_types::NodeStatus;
use crate::rpc::{RpcContext, errors::rpc_error};
use eve_storage::state::read_state_service;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;
use std::sync::atomic::Ordering;
pub(crate) fn read_node_status(context: &RpcContext) -> Result<Value, ErrorObjectOwned> {
    let head =
        read_state_service(&context.service).map_err(|e| rpc_error(-32000, e.to_string()))?;
    let status = NodeStatus {
        applied_height: head.commit().target.height,
        durable_height: head.commit().target.height,
        authenticated_height: None,
        finalized_height: None,
        verification_mode: "LOCAL_DEV_UNAUTHENTICATED",
        authenticated_finality: false,
        ready: context.healthy.load(Ordering::Acquire),
        peer_count: 0,
        lag: 0,
        database_sequence: head.sequence(),
        zone_id: context.zone.0,
    };
    serde_json::to_value(status).map_err(|e| rpc_error(-32603, e.to_string()))
}

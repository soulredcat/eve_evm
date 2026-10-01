// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{RpcContext, RpcEvent, encoding::encode_block_logs};
use anyhow::Result;
use eve_storage::state::RetainedBlockProjection;
use serde_json::Value;
use std::sync::Arc;
/// Share immutable payload and keep its logical byte reservation until the last subscriber/ring reference drops.
pub(crate) fn build_rpc_event(
    context: &RpcContext,
    block: &RetainedBlockProjection,
) -> Result<Arc<RpcEvent>> {
    let head = serde_json::to_value(alloy_rpc_types_eth::Header::new(block.block.header.clone()))?;
    let logs = match encode_block_logs(block) {
        Ok(logs) => logs,
        Err(_) => {
            return Ok(Arc::new(RpcEvent {
                head: Value::Null,
                logs: Vec::new(),
                fault: Some("SUBSCRIPTION_EVENT_LIMIT: fetch durable HTTP history"),
                _bytes: None,
            }));
        }
    };
    let encoded = serde_json::to_vec(&head)?
        .len()
        .checked_add(serde_json::to_vec(&logs)?.len())
        .ok_or_else(|| anyhow::anyhow!("event byte accounting overflow"))?;
    if encoded > 4 * 1_048_576 || logs.len() > 10_000 {
        return Ok(Arc::new(RpcEvent {
            head: Value::Null,
            logs: Vec::new(),
            fault: Some("SUBSCRIPTION_EVENT_LIMIT: fetch durable HTTP history"),
            _bytes: None,
        }));
    }
    let charge = u32::try_from(
        encoded
            .checked_mul(3)
            .ok_or_else(|| anyhow::anyhow!("event reservation overflow"))?
            .div_ceil(1024),
    )?;
    let lease = Arc::clone(&context.bytes).try_acquire_many_owned(charge);
    match lease {
        Ok(lease) => Ok(Arc::new(RpcEvent {
            head,
            logs,
            fault: None,
            _bytes: Some(lease),
        })),
        Err(_) => Ok(Arc::new(RpcEvent {
            head: Value::Null,
            logs: Vec::new(),
            fault: Some("SUBSCRIPTION_BYTE_CAPACITY: fetch durable HTTP history"),
            _bytes: None,
        })),
    }
}

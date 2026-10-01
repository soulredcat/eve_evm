// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{
    RpcContext,
    errors::rpc_error,
    history::{decode_log_filter, matches_log},
};
use jsonrpsee::{PendingSubscriptionSink, core::SubscriptionError, types::Params};
use serde_json::Value;
use std::sync::Arc;
pub(crate) async fn stream_subscription(
    params: Params<'_>,
    pending: PendingSubscriptionSink,
    context: Arc<RpcContext>,
) -> Result<(), SubscriptionError> {
    let params = match params.parse::<Vec<Value>>() {
        Ok(params) => params,
        Err(error) => {
            pending.reject(error).await;
            return Ok(());
        }
    };
    let kind = params.first().and_then(Value::as_str).unwrap_or("");
    if (kind != "newHeads" && kind != "logs") || params.len() > if kind == "logs" { 2 } else { 1 } {
        pending
            .reject(rpc_error(-32602, "supported subscriptions: newHeads, logs"))
            .await;
        return Ok(());
    }
    let filter =
        match decode_log_filter(params.get(1).unwrap_or(&Value::Object(Default::default()))) {
            Ok(filter) => filter,
            Err(error) => {
                pending.reject(error).await;
                return Ok(());
            }
        };
    if filter.from.is_some() || filter.to.is_some() || filter.block_hash.is_some() {
        pending
            .reject(rpc_error(
                -32602,
                "subscription supports address and topics filters",
            ))
            .await;
        return Ok(());
    }
    let _permit = match Arc::clone(&context.subscriptions).try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            pending
                .reject(rpc_error(-32005, "global subscription capacity exceeded"))
                .await;
            return Ok(());
        }
    };
    let mut events = context.events.subscribe();
    let sink = pending.accept().await?;
    loop {
        let event = tokio::select! {
            _ = sink.closed() => return Ok(()),
            event = events.recv() => event.map_err(|_| SubscriptionError::from("SUBSCRIPTION_LAGGED: reconnect and query durable history"))?,
        };
        if let Some(fault) = event.fault {
            return Err(SubscriptionError::from(fault));
        }
        if kind == "newHeads" {
            super::send_subscription_value::send_subscription_value(&context, &sink, &event.head)
                .await?;
        } else {
            for log in &event.logs {
                if matches_log(&filter, log) {
                    super::send_subscription_value::send_subscription_value(&context, &sink, log)
                        .await?;
                }
            }
        }
    }
}

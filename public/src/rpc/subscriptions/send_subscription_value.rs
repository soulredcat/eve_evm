// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::rpc::{RpcContext, encoding::estimate_json_value_bytes};
use jsonrpsee::{SubscriptionMessage, SubscriptionSink, core::SubscriptionError};
use serde_json::Value;
use std::sync::Arc;
pub(crate) async fn send_subscription_value(
    context: &RpcContext,
    sink: &SubscriptionSink,
    value: &Value,
) -> Result<(), SubscriptionError> {
    let bytes = estimate_json_value_bytes(value, 63_488).map_err(|_| {
        SubscriptionError::from("SUBSCRIPTION_FRAME_LIMIT: fetch durable HTTP history")
    })?;
    let charge = u32::try_from(
        bytes
            .checked_mul(3)
            .and_then(|bytes| bytes.checked_add(2048))
            .ok_or_else(|| SubscriptionError::from("notification accounting overflow"))?
            .div_ceil(1024),
    )
    .map_err(|_| SubscriptionError::from("notification reservation overflow"))?;
    let _lease = Arc::clone(&context.bytes)
        .try_acquire_many_owned(charge)
        .map_err(|_| {
            SubscriptionError::from("SUBSCRIPTION_BYTE_CAPACITY: fetch durable HTTP history")
        })?;
    let message = SubscriptionMessage::new("eth_subscription", sink.subscription_id(), value)?;
    sink.send_timeout(message, std::time::Duration::from_secs(2))
        .await
        .map_err(|_| {
            SubscriptionError::from(
                "SUBSCRIPTION_BACKPRESSURE: reconnect and fetch durable HTTP history",
            )
        })?;
    Ok(())
}

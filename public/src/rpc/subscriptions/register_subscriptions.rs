// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::stream_subscription::stream_subscription;
use crate::rpc::RpcContext;
use anyhow::Result;
use jsonrpsee::RpcModule;
use std::sync::Arc;
pub(crate) fn register_subscriptions(module: &mut RpcModule<Arc<RpcContext>>) -> Result<()> {
    module.register_subscription(
        "eth_subscribe",
        "eth_subscription",
        "eth_unsubscribe",
        |params, pending, context, _| async move {
            stream_subscription(params, pending, Arc::clone(&context)).await
        },
    )?;
    Ok(())
}

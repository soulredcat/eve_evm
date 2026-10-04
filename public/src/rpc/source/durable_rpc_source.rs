// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RpcStateSource;
use crate::rpc::{RpcContext, errors::rpc_error};
use eve_storage::state::{StateReader, StateService};
use jsonrpsee::types::ErrorObjectOwned;

pub(crate) fn durable_rpc_source(
    context: &RpcContext,
) -> Result<(&StateService, &StateReader), ErrorObjectOwned> {
    match &context.source {
        RpcStateSource::Durable { service, reader } => Ok((service.as_ref(), reader.as_ref())),
        RpcStateSource::Applied { .. } => Err(rpc_error(
            -32001,
            "GAP: durable historical RPC source is unavailable for this applied view",
        )),
    }
}

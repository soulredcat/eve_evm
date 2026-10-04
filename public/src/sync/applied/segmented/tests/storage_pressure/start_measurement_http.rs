// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{HttpRpcClients, MeasurementContract, MeasurementServers};
use crate::rpc::{RpcContext, RpcListenerAddresses};
use jsonrpsee::http_client::HttpClientBuilder;
use std::{sync::Arc, time::Duration};

pub(super) async fn start_measurement_http(
    context: Arc<RpcContext>,
    contract: &MeasurementContract,
) -> (HttpRpcClients, MeasurementServers) {
    let (http, ws, address, _) = crate::runtime::start_rpc_servers(
        RpcListenerAddresses {
            http_address: "127.0.0.1:0".parse().unwrap(),
            ws_address: "127.0.0.1:0".parse().unwrap(),
        },
        context,
    )
    .await
    .unwrap();
    let uri = format!("http://{address}");
    let servers = MeasurementServers {
        handles: Some((http, ws)),
    };
    let clients = HttpRpcClients {
        balance: Arc::new(
            HttpClientBuilder::default()
                .request_timeout(Duration::from_millis(contract.rpc_operation_timeout_ms))
                .max_request_size(65_536)
                .max_response_size(65_536)
                .max_concurrent_requests(1)
                .build(&uri)
                .unwrap(),
        ),
        call: Arc::new(
            HttpClientBuilder::default()
                .request_timeout(Duration::from_millis(contract.rpc_operation_timeout_ms))
                .max_request_size(65_536)
                .max_response_size(65_536)
                .max_concurrent_requests(1)
                .build(&uri)
                .unwrap(),
        ),
    };
    (clients, servers)
}

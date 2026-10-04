// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{net::SocketAddr, time::Duration};

/// Explicit configured endpoint. Plain TCP is supported only on loopback for local development.
#[derive(Clone, Copy)]
pub struct NativeRpcConfig {
    pub address: SocketAddr,
    pub timeout: Duration,
    pub maximum_request_bytes: usize,
    pub maximum_response_bytes: usize,
}

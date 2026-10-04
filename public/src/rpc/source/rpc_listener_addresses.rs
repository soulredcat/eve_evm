// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::net::SocketAddr;

pub(crate) struct RpcListenerAddresses {
    pub http_address: SocketAddr,
    pub ws_address: SocketAddr,
}

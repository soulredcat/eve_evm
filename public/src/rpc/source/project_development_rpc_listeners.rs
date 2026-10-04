// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RpcListenerAddresses;
use crate::runtime::DevelopmentPublicConfig;

pub(super) fn project_development_rpc_listeners(
    config: &DevelopmentPublicConfig,
) -> RpcListenerAddresses {
    RpcListenerAddresses {
        http_address: config.http_address,
        ws_address: config.ws_address,
    }
}

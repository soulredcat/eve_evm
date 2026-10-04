// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    RpcListenerAddresses, project_development_rpc_listeners::project_development_rpc_listeners,
};
use crate::runtime::DevelopmentPublicConfig;

impl std::convert::From<&DevelopmentPublicConfig> for RpcListenerAddresses {
    fn from(config: &DevelopmentPublicConfig) -> Self {
        project_development_rpc_listeners(config)
    }
}

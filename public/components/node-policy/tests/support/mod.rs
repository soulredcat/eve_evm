// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::B256;
use eve_protocol_config::network::{NetworkProfileBinding, SecurityProfile};

pub fn binding() -> NetworkProfileBinding {
    NetworkProfileBinding {
        genesis_hash: B256::repeat_byte(1),
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        protocol_version: 1,
        profile: SecurityProfile::ClassicalDev,
        activation_height: 1,
        key_epoch: 0,
    }
}

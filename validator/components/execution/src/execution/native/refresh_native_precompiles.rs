// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::native::SYSTEM_INTERFACE_ADDRESS;
use revm::{handler::EthPrecompiles, primitives::hardfork::SpecId};

use super::InactiveNativeProvider;

pub(crate) fn refresh_native_precompiles<S: Into<SpecId>>(
    provider: &mut InactiveNativeProvider,
    spec: S,
) -> bool {
    let spec = spec.into();
    if provider.ethereum.spec == spec {
        return false;
    }
    provider.ethereum = EthPrecompiles::new(spec);
    provider.warmed = provider.ethereum.warm_addresses().clone();
    provider.warmed.insert(SYSTEM_INTERFACE_ADDRESS);
    true
}

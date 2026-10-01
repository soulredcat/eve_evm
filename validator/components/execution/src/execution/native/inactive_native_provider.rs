// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::native::SYSTEM_INTERFACE_ADDRESS;
use revm::{handler::EthPrecompiles, primitives::hardfork::SpecId};

use super::InactiveNativeProvider;

pub(crate) fn inactive_native_provider() -> InactiveNativeProvider {
    let ethereum = EthPrecompiles::new(SpecId::SHANGHAI);
    let mut warmed = ethereum.warm_addresses().clone();
    warmed.insert(SYSTEM_INTERFACE_ADDRESS);
    InactiveNativeProvider { ethereum, warmed }
}

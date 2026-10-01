// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod inactive_native_provider;
mod native_warm_addresses;
mod provider_adapter;
mod refresh_native_precompiles;
mod reject_inactive_native_call;

pub(crate) use inactive_native_provider::inactive_native_provider;
pub(crate) use provider_adapter::InactiveNativeProvider;
pub use reject_inactive_native_call::NATIVE_INTERFACE_INACTIVE_REVERT_DATA;

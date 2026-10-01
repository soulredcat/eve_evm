// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use revm::primitives::AddressSet;

use super::InactiveNativeProvider;

pub(crate) fn native_warm_addresses(provider: &InactiveNativeProvider) -> &AddressSet {
    &provider.warmed
}

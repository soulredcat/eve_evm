// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_evm::decode_signed_transaction;
use eve_state::Address;

pub fn sender() -> Address {
    decode_signed_transaction(&super::signed_transaction(), 31_337, 131_072)
        .unwrap()
        .sender()
}

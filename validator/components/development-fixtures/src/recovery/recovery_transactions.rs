// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::signed_transaction;
use crate::transactions::signed_transaction_with_nonce;
use eve_state::Bytes;
pub(super) fn recovery_transactions(height: i64, nonempty_tail: bool) -> Vec<Bytes> {
    match height {
        1 => vec![signed_transaction()],
        2 if nonempty_tail => vec![signed_transaction_with_nonce(1)],
        _ => Vec::new(),
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::build_native_block_response;
use crate::support::{
    native_decoding::decode_native_block, transactions::types::LocatedTransaction,
};

#[cfg(test)]
pub(in super::super) fn build_located_transaction(height: i64) -> LocatedTransaction {
    LocatedTransaction {
        block: decode_native_block(&build_native_block_response(height, &[vec![1, 2, 3]])).unwrap(),
        index: 0,
    }
}

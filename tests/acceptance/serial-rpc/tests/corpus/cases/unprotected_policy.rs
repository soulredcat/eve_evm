// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::corpus::{
    decode_reference_transaction::decode_reference_transaction,
    load_pinned_cases::load_pinned_cases,
};
use eve_evm::decode_signed_transaction;

#[test]
fn te01_all_88_unprotected_corpus_envelopes_are_rejected_by_eve() {
    let mut rejected = 0;
    for (name, case) in load_pinned_cases() {
        let raw = &case.post["Shanghai"][0].txbytes;
        if decode_reference_transaction(raw)
            .is_ok_and(|(transaction, _)| transaction.chain_id.is_none())
        {
            assert_eq!(
                decode_signed_transaction(raw, 1, 128 * 1024).unwrap_err(),
                eve_evm::TransactionValidationError::WrongChain,
                "{name}"
            );
            rejected += 1;
        }
    }
    assert_eq!(rejected, 88);
}

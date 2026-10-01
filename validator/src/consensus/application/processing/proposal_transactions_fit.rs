// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::consensus::certificates::hash_transaction_data;

/// Peer-supplied deterministic envelope limits reject a candidate without fencing local actors.
pub(in crate::consensus::application) fn proposal_transactions_fit(
    transactions: &[Vec<u8>],
) -> bool {
    hash_transaction_data(transactions).is_ok()
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Roster, accept, reject_unchanged};
use eve_consensus_comet::consensus::{certificates::CertificateError, history::HistoryError};
use sha2::{Digest, Sha256};

#[test]
fn authenticated_header_requires_its_exact_raw_transaction_data() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    let mut first = roster.frame(1, None, &roster);
    // Independently calculate the one-transaction native tree: SHA256(0x00 || txID).
    let tx_id = Sha256::digest(&first.transactions[0]);
    let mut leaf = Sha256::new();
    leaf.update([0]);
    leaf.update(tx_id);
    assert_eq!(first.header.data_hash, leaf.finalize().to_vec());
    first.transactions[0][0] ^= 1;
    assert_eq!(
        reject_unchanged(&mut history, &first, &roster),
        HistoryError::WrongTransactionData
    );
    first.transactions.clear();
    assert_eq!(
        reject_unchanged(&mut history, &first, &roster),
        HistoryError::WrongTransactionData
    );
    accept(&mut history, &roster.frame(1, None, &roster), &roster);
}

#[test]
fn a_quorum_signed_header_with_wrong_data_hash_is_still_rejected() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    let mut first = roster.frame(1, None, &roster);
    first.header.data_hash[0] ^= 1;
    roster.resign(&mut first);
    assert_eq!(
        reject_unchanged(&mut history, &first, &roster),
        HistoryError::WrongTransactionData
    );
    accept(&mut history, &roster.frame(1, None, &roster), &roster);
}

#[test]
fn exact_two_thirds_and_corrupt_signatures_do_not_create_verified_history() {
    let roster = Roster::new(1, 3);
    let mut history = roster.history();
    let mut first = roster.frame(1, None, &roster);
    first.commit.signatures[2] = eve_consensus_comet::wire::tendermint::types::CommitSig {
        block_id_flag: 1,
        validator_address: Vec::new(),
        timestamp: Some(prost_types::Timestamp {
            seconds: -62_135_596_800,
            nanos: 0,
        }),
        signature: Vec::new(),
    };
    assert_eq!(
        reject_unchanged(&mut history, &first, &roster),
        HistoryError::Certificate(CertificateError::InsufficientVotingPower)
    );
    let mut corrupt = roster.frame(1, None, &roster);
    corrupt.commit.signatures[0].signature[0] ^= 1;
    reject_unchanged(&mut history, &corrupt, &roster);
    accept(&mut history, &roster.frame(1, None, &roster), &roster);
}

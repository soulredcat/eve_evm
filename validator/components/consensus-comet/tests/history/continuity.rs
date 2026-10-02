// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Roster, accept, reject_unchanged};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::CertificateError,
    history::{HistoryError, initialize_trusted_header_history},
};

#[test]
fn missing_reordered_repeated_and_conflicting_heights_never_advance_history() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    let first = roster.frame(1, None, &roster);
    let second = roster.frame(2, Some(first.block_id.clone()), &roster);
    assert_eq!(
        reject_unchanged(&mut history, &second, &roster),
        HistoryError::WrongSuccessorHeight
    );
    accept(&mut history, &first, &roster);
    reject_unchanged(&mut history, &first, &roster);
    let mut conflict = roster.frame(1, None, &roster);
    conflict.header.app_hash[0] ^= 1;
    roster.resign(&mut conflict);
    reject_unchanged(&mut history, &conflict, &roster);
    let skipped = roster.frame(3, Some(first.block_id.clone()), &roster);
    reject_unchanged(&mut history, &skipped, &roster);
    accept(&mut history, &second, &roster);
    reject_unchanged(&mut history, &first, &roster);
}

#[test]
fn parent_header_hash_and_part_set_identity_are_both_required() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    let first = roster.frame(1, None, &roster);
    accept(&mut history, &first, &roster);
    let mut wrong_hash = first.block_id.clone();
    wrong_hash.hash[0] ^= 1;
    reject_unchanged(
        &mut history,
        &roster.frame(2, Some(wrong_hash), &roster),
        &roster,
    );
    let mut wrong_parts = first.block_id.clone();
    wrong_parts.part_set_header.as_mut().unwrap().total += 1;
    reject_unchanged(
        &mut history,
        &roster.frame(2, Some(wrong_parts), &roster),
        &roster,
    );
    let missing = roster.frame(2, None, &roster);
    reject_unchanged(&mut history, &missing, &roster);
    accept(
        &mut history,
        &roster.frame(2, Some(first.block_id), &roster),
        &roster,
    );
}

#[test]
fn a_genesis_successor_with_a_non_nil_parent_is_rejected() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    let parent = roster.frame(1, None, &roster).block_id;
    reject_unchanged(
        &mut history,
        &roster.frame(1, Some(parent), &roster),
        &roster,
    );
    accept(&mut history, &roster.frame(1, None, &roster), &roster);
}

#[test]
fn nonincreasing_consensus_time_cannot_advance_history() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    let first = roster.frame(1, None, &roster);
    accept(&mut history, &first, &roster);
    let mut stale = roster.frame(2, Some(first.block_id.clone()), &roster);
    stale.header.time = first.header.time;
    roster.resign(&mut stale);
    reject_unchanged(&mut history, &stale, &roster);
    stale.header.time.as_mut().unwrap().seconds -= 1;
    roster.resign(&mut stale);
    reject_unchanged(&mut history, &stale, &roster);
    accept(
        &mut history,
        &roster.frame(2, Some(first.block_id), &roster),
        &roster,
    );
}

#[test]
fn target_part_set_cannot_be_substituted_for_the_certified_full_block_id() {
    let roster = Roster::new(1, 4);
    let mut history = roster.history();
    let mut first = roster.frame(1, None, &roster);
    first.block_id.part_set_header.as_mut().unwrap().hash[0] ^= 1;
    assert_eq!(
        reject_unchanged(&mut history, &first, &roster),
        HistoryError::Certificate(CertificateError::WrongBlock)
    );
    accept(&mut history, &roster.frame(1, None, &roster), &roster);
}

#[test]
fn a_locally_trusted_maximum_height_cannot_wrap_into_a_successor() {
    let roster = Roster::new(1, 4);
    let mut anchor = roster.frame(7, None, &roster);
    anchor.header.height = i64::MAX;
    roster.resign(&mut anchor);
    let mut history = initialize_trusted_header_history(
        crate::support::CHAIN,
        &anchor.header,
        &anchor.block_id,
        ConsensusAuthenticationRequirement::ClassicalDev,
    )
    .unwrap();
    assert_eq!(
        reject_unchanged(&mut history, &roster.frame(1, None, &roster), &roster),
        HistoryError::HeightOverflow
    );
}

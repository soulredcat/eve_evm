// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Roster, accept, reject_unchanged};

#[test]
fn finalize_height_one_set_update_activates_at_header_height_three() {
    let old = Roster::new(1, 4);
    let new = Roster::new(11, 4);
    let mut history = old.history();
    let first = old.frame(1, None, &old);
    accept(&mut history, &first, &old);
    let premature = new.frame(2, Some(first.block_id.clone()), &new);
    reject_unchanged(&mut history, &premature, &new);
    // FinalizeBlock(1)'s changed next-set hash appears at H2; voters switch at H3.
    let second = old.frame(2, Some(first.block_id), &new);
    accept(&mut history, &second, &old);
    let retired = old.frame(3, Some(second.block_id.clone()), &old);
    reject_unchanged(&mut history, &retired, &old);
    accept(
        &mut history,
        &new.frame(3, Some(second.block_id), &new),
        &new,
    );
}

#[test]
fn a_peer_supplied_forged_current_roster_cannot_replace_genesis_trust() {
    let trusted = Roster::new(1, 4);
    let attacker = Roster::new(11, 4);
    let mut history = trusted.history();
    reject_unchanged(&mut history, &attacker.frame(1, None, &attacker), &attacker);
    accept(&mut history, &trusted.frame(1, None, &trusted), &trusted);
}

#[test]
fn reordered_roster_or_unsigned_next_set_change_cannot_advance_history() {
    let roster = Roster::new(1, 4);
    let changed = Roster::new(11, 4);
    let mut history = roster.history();
    let mut first = roster.frame(1, None, &roster);
    let mut reversed = Roster::new(1, 4);
    reversed.validators.reverse();
    reject_unchanged(&mut history, &first, &reversed);
    first.header.next_validators_hash =
        eve_consensus_comet::consensus::certificates::hash_validator_set(&changed.validators)
            .unwrap()
            .to_vec();
    // Retain the original signed block identity: this next-set mutation has no certificate.
    reject_unchanged(&mut history, &first, &roster);
    accept(&mut history, &roster.frame(1, None, &roster), &roster);
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{Frame, Roster};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    history::{HistoryError, NativeHistoryVerifier, verify_next_native_header},
};

pub fn accept(history: &mut NativeHistoryVerifier, frame: &Frame, roster: &Roster) {
    let verified = verify_next_native_header(
        history,
        &frame.block_id,
        &frame.header,
        &frame.commit,
        &roster.validators,
        &frame.transactions,
    )
    .unwrap();
    assert_eq!(verified.header(), &frame.header);
    assert_eq!(verified.block_id(), &frame.block_id);
    assert_eq!(
        verified.authentication(),
        ConsensusAuthenticationRequirement::ClassicalDev
    );
    assert_eq!(
        verified.signed_voting_power(),
        i64::try_from(roster.validators.len()).unwrap()
    );
    assert_eq!(
        verified.total_voting_power(),
        i64::try_from(roster.validators.len()).unwrap()
    );
    assert_eq!(history.height(), frame.header.height);
    assert_eq!(history.block_id(), Some(&frame.block_id));
}

pub fn reject_unchanged(
    history: &mut NativeHistoryVerifier,
    frame: &Frame,
    roster: &Roster,
) -> HistoryError {
    let before = (
        history.chain_id().to_owned(),
        history.height(),
        history.block_id().cloned(),
        history.authentication(),
    );
    let error = verify_next_native_header(
        history,
        &frame.block_id,
        &frame.header,
        &frame.commit,
        &roster.validators,
        &frame.transactions,
    )
    .err()
    .unwrap();
    assert_eq!(
        before,
        (
            history.chain_id().to_owned(),
            history.height(),
            history.block_id().cloned(),
            history.authentication()
        )
    );
    error
}

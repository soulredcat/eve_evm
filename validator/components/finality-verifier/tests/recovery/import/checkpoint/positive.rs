// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execution, lookahead, session};
use crate::recovery_support::{CLONE_BYTES, recovery_chain};
use eve_finality_verifier::{
    checkpoint_anchor, checkpoint_commit, finish_authenticated_checkpoint, imported_state_anchor,
    imported_state_commit, initialize_authenticated_import, into_imported_checkpoint_state,
    into_imported_state, prepare_authenticated_import, verify_checkpoint_witness,
};
use eve_state::development_state_budget;
use std::sync::Arc;

#[test]
fn streamed_real_native_witnesses_authenticate_complete_nonempty_checkpoint_at_h_plus_one() {
    let chain = recovery_chain();
    let parent =
        initialize_authenticated_import(&chain.genesis, &development_state_budget()).unwrap();
    let (mut progress, reserved) = session(&parent, &chain);
    verify_checkpoint_witness(&mut progress, &execution(&chain, 1), reserved).unwrap();
    verify_checkpoint_witness(&mut progress, &execution(&chain, 2), reserved).unwrap();
    verify_checkpoint_witness(&mut progress, &lookahead(&chain), reserved).unwrap();
    let checkpoint = finish_authenticated_checkpoint(progress).unwrap();
    assert_eq!(checkpoint_commit(&checkpoint), &chain.commits[2]);
    assert_eq!(checkpoint_anchor(&checkpoint).execution_height(), 2);
    assert_eq!(checkpoint_anchor(&checkpoint).consensus_height(), 3);
    let imported = into_imported_checkpoint_state(checkpoint);
    assert_eq!(imported_state_commit(&imported), &chain.commits[2]);
    assert_eq!(
        imported_state_anchor(&imported)
            .unwrap()
            .consensus_block_id(),
        &chain.frames[2].id
    );
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn actual_imported_parent_reuses_only_its_exact_certified_lookahead_and_preserves_known_history() {
    let chain = recovery_chain();
    let genesis =
        initialize_authenticated_import(&chain.genesis, &development_state_budget()).unwrap();
    let parent = into_imported_state(
        prepare_authenticated_import(
            &genesis,
            Arc::new(super::super::support::input(&chain, 1)),
            &development_state_budget(),
            CLONE_BYTES,
        )
        .unwrap(),
    );
    let (mut progress, reserved) = session(&parent, &chain);
    verify_checkpoint_witness(&mut progress, &execution(&chain, 2), reserved).unwrap();
    verify_checkpoint_witness(&mut progress, &lookahead(&chain), reserved).unwrap();
    let checkpoint = finish_authenticated_checkpoint(progress).unwrap();
    assert_eq!(
        checkpoint_commit(&checkpoint).state.block_hashes,
        chain.commits[2].state.block_hashes
    );
    assert_eq!(imported_state_commit(&parent), &chain.commits[1]);
}

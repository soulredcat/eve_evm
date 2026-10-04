// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execution, limits, session};
use crate::recovery_support::recovery_chain;
use eve_finality_verifier::{
    CheckpointError, CheckpointLimits, begin_authenticated_checkpoint,
    initialize_authenticated_import, required_checkpoint_reservation, verify_checkpoint_witness,
};
use eve_state::development_state_budget;
use std::sync::Arc;

#[test]
fn reservation_and_witness_limits_refuse_before_stream_progress_without_raising_defaults() {
    let chain = recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let (mut progress, reserved) = session(&parent, &chain);
    assert!(matches!(
        verify_checkpoint_witness(&mut progress, &execution(&chain, 1), reserved - 1),
        Err(CheckpointError::ReservationTooSmall)
    ));
    verify_checkpoint_witness(&mut progress, &execution(&chain, 1), reserved).unwrap();
    assert!(matches!(
        begin_authenticated_checkpoint(
            &parent,
            Arc::new(chain.commits[2].clone()),
            &budget,
            limits(),
            reserved - 1
        ),
        Err(CheckpointError::ReservationTooSmall)
    ));
    let small = CheckpointLimits {
        maximum_height_gap: 1,
        maximum_witness_bytes: limits().maximum_witness_bytes,
    };
    assert!(
        begin_authenticated_checkpoint(
            &parent,
            Arc::new(chain.commits[2].clone()),
            &budget,
            small,
            reserved
        )
        .is_err()
    );
    let overflow = CheckpointLimits {
        maximum_height_gap: 8,
        maximum_witness_bytes: usize::MAX,
    };
    assert!(matches!(
        required_checkpoint_reservation(&parent, &chain.commits[2], &budget, overflow),
        Err(CheckpointError::Overflow)
    ));
}

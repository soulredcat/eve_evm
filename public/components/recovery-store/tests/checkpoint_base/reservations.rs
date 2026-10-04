// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::*;
use eve_storage::records::segmented::checkpoints::*;

#[test]
fn preparation_encoding_and_actual_read_require_admitted_reservations() {
    assert!(matches!(
        prepare_checkpoint_base_target(&version(4), &limits(), reservation() - 1),
        Err(CheckpointBaseError::InsufficientReservation)
    ));
    let target = prepared(4);
    let parent = inert_parent();
    assert!(matches!(
        encode_checkpoint_base(
            metadata(parent, parent.cursor, 4),
            &target,
            &limits(),
            reservation() - 1
        ),
        Err(CheckpointBaseError::InsufficientReservation)
    ));
    let directory = tempfile::tempdir().unwrap();
    let mut repository = open(directory.path());
    let parent = bootstrap(&repository);
    let expected = metadata(parent, parent.cursor, 4);
    let cursor = append(&mut repository, encode(expected, &target));
    assert!(matches!(
        read_checkpoint_base_membership(
            &repository,
            cursor,
            expected,
            &target,
            &limits(),
            membership_charge(&repository) - 1
        ),
        Err(CheckpointBaseError::InsufficientReservation)
    ));
}

#[test]
fn invalid_ceiling_oversized_target_and_read_arithmetic_reject_before_owned_read() {
    for maximum_payload_bytes in [0, 396, CHECKPOINT_BASE_MAX_PAYLOAD_BYTES + 1, usize::MAX] {
        assert!(matches!(
            required_checkpoint_base_encoding_reservation(&CheckpointBaseLimits {
                maximum_payload_bytes
            }),
            Err(CheckpointBaseError::InvalidLimits)
        ));
    }
    let mut oversized = version(4);
    oversized.identity.network_name = "x".repeat(1_025);
    assert!(matches!(
        prepare_checkpoint_base_target(&oversized, &limits(), reservation()),
        Err(CheckpointBaseError::LimitExceeded)
    ));
    let mut actual_budget = budget();
    actual_budget.maximum_read_bytes = usize::MAX;
    assert!(matches!(
        required_checkpoint_base_membership_reservation(&actual_budget, &limits()),
        Err(CheckpointBaseError::ArithmeticOverflow)
    ));
    let target = prepared(4);
    let parent = inert_parent();
    let constrained = CheckpointBaseLimits {
        maximum_payload_bytes: 397,
    };
    let charge = required_checkpoint_base_encoding_reservation(&constrained).unwrap();
    assert!(matches!(
        encode_checkpoint_base(
            metadata(parent, parent.cursor, 4),
            &target,
            &constrained,
            charge
        ),
        Err(CheckpointBaseError::LimitExceeded)
    ));
}

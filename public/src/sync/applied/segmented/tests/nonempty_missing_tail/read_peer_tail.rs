// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    AppliedReader, AppliedWorkingReservation, reserve_applied_working,
    resources::estimated_repository_read_charge,
};
use eve_storage::records::{
    OpaqueRecordRepository, opaque_record_budget, opaque_record_cursor, read_opaque_record,
};
/// Lease precedes actual bounded repository allocation and remains with bytes until caller finishes proof/application.
pub(super) fn read_peer_tail(
    repository: &OpaqueRecordRepository,
    reader: &AppliedReader,
) -> (AppliedWorkingReservation, Option<Vec<u8>>) {
    let lease = reserve_applied_working(
        reader,
        estimated_repository_read_charge(&opaque_record_budget(repository)).unwrap(),
    )
    .unwrap();
    let head = opaque_record_cursor(repository).unwrap();
    let bytes = read_opaque_record(repository, 4).unwrap().map(|record| {
        assert_eq!(record.sequence, 4);
        assert_eq!(record.content_hash, head.content_hash);
        record.payload
    });
    (lease, bytes)
}

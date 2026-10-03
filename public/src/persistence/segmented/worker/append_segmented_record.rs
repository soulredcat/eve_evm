// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::{SegmentedError, types::WorkerState},
    reserve_scratch::reserve_scratch,
};
use eve_storage::records::{
    OpaqueRecordAck, OpaqueRecordCursor, OpaqueRecordRepository, compare_and_append_opaque_records,
};
use std::sync::Arc;
pub(super) fn append_segmented_record(
    repository: &mut OpaqueRecordRepository,
    state: &Arc<WorkerState>,
    expected: OpaqueRecordCursor,
    payload: &Vec<u8>,
    record_index: usize,
) -> Result<OpaqueRecordAck, SegmentedError> {
    let _scratch = reserve_scratch(state, payload.len())?;
    #[cfg(test)]
    super::super::tests::before_record(state, record_index);
    #[cfg(not(test))]
    let _ = record_index;
    compare_and_append_opaque_records(repository, expected, std::slice::from_ref(payload))
        .map_err(|_| SegmentedError::StorageFailed)
}

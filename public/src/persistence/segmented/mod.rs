// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Version 2 retained part buffers and exclusive sequential physical persistence.
//! Local markers do not authenticate execution, finality or peer availability.
mod planning;
mod pool;
mod reservation;
mod sealing;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use tests::pause as install_segmented_record_pause;
mod types;
mod views;
mod worker;

pub use planning::{plan_segmented_batch, plan_segmented_layout};
pub use pool::{
    create_segmented_part_pool, observe_segmented_parts, required_segmented_metadata_reservation,
};
pub use reservation::{
    bind_segmented_reservation, reserve_segmented_batch, reserve_segmented_layout,
};
pub use sealing::write_and_seal_segmented_batch;
pub use types::{
    RejectedSegmentedBatch, RejectedSegmentedReservation, SealedSegmentedBatch,
    SegmentedBatchLayout, SegmentedBatchPlan, SegmentedBatchReservation, SegmentedError,
    SegmentedLogicalAck, SegmentedPartObservation, SegmentedPartPool, SegmentedTail,
    SegmentedTicket, SegmentedWorker, SegmentedWorkerObservation, SegmentedWorkerShutdown,
    UnboundSegmentedReservation,
};
pub use views::{
    segmented_batch_expected_cursor, segmented_batch_identity, segmented_batch_marker_cursor,
    segmented_batch_references, segmented_batch_target_binding, segmented_part_bytes,
};
pub use worker::{
    finish_segmented_worker, observe_segmented_worker, start_segmented_worker,
    try_receive_segmented_ack, try_submit_segmented_batch,
};

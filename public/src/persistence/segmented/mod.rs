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
mod types;
mod views;
mod worker;

pub use planning::plan_segmented_batch;
pub use pool::{create_segmented_part_pool, observe_segmented_parts};
pub use reservation::reserve_segmented_batch;
pub use sealing::write_and_seal_segmented_batch;
pub use types::{
    RejectedSegmentedBatch, SealedSegmentedBatch, SegmentedBatchPlan, SegmentedBatchReservation,
    SegmentedError, SegmentedLogicalAck, SegmentedPartObservation, SegmentedPartPool,
    SegmentedTail, SegmentedTicket, SegmentedWorker, SegmentedWorkerObservation,
    SegmentedWorkerShutdown,
};
pub use views::segmented_part_bytes;
pub use worker::{
    finish_segmented_worker, observe_segmented_worker, start_segmented_worker,
    try_receive_segmented_ack, try_submit_segmented_batch,
};

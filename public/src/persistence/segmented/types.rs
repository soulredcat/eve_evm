// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod batch_types;
mod pool_types;
mod worker_types;
pub(super) use batch_types::{AllocatedPart, SealedBatch};
pub use batch_types::{
    RejectedSegmentedReservation, SealedSegmentedBatch, SegmentedBatchLayout, SegmentedBatchPlan,
    SegmentedBatchReservation, UnboundSegmentedReservation,
};
pub(super) use pool_types::{
    MetadataLease, PartAccounting, PartLease, PartSlot, WorkerLifetimeLease,
};
pub use pool_types::{SegmentedPartObservation, SegmentedPartPool};
pub(super) use worker_types::{Admission, AdmittedBatch, Request, ScratchLease, WorkerState};
pub use worker_types::{
    RejectedSegmentedBatch, SegmentedLogicalAck, SegmentedTail, SegmentedTicket, SegmentedWorker,
    SegmentedWorkerObservation, SegmentedWorkerShutdown,
};
pub(super) const PARTS: usize = 4;
pub(super) const SEGMENTS: usize = 6;
pub(super) const BATCHES: usize = 2;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmentedError {
    InvalidConfiguration,
    InvalidPlan,
    Overflow,
    Capacity,
    Allocation,
    AccountingUnavailable,
    UnexpectedCapacity,
    Codec,
    ForeignPool,
    WrongCursor,
    AlreadySubmitted,
    WorkerAlreadyActive,
    QueueFull,
    QueueAged,
    LogicalLag,
    Closed,
    StorageFailed,
    AckMismatch,
    AckLost,
    WorkerPanicked,
    SpawnFailed,
}

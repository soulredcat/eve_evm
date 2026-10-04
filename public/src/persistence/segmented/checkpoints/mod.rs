// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Local checkpoint-base persistence on the same bounded sequential worker.
//! Artifact verification and authenticated activation remain caller responsibilities.
mod checkpoint_record_cursor;
mod checkpoint_record_metadata;
mod dispatch_checkpoint_request;
mod persist_checkpoint_base;
mod seal_checkpoint_base_record;
mod start_segmented_worker_from_checkpoint;
mod try_receive_checkpoint_ack;
mod try_submit_checkpoint_base;
pub(in crate::persistence::segmented) mod types;
pub use checkpoint_record_cursor::checkpoint_record_cursor;
pub use checkpoint_record_metadata::checkpoint_record_metadata;
pub(in crate::persistence::segmented) use dispatch_checkpoint_request::dispatch_checkpoint_request;
pub use seal_checkpoint_base_record::seal_checkpoint_base_record;
pub use start_segmented_worker_from_checkpoint::start_segmented_worker_from_checkpoint;
pub use try_receive_checkpoint_ack::try_receive_checkpoint_ack;
pub use try_submit_checkpoint_base::try_submit_checkpoint_base;
pub use types::{
    CheckpointAck, CheckpointTail, CheckpointTicket, RejectedCheckpointRecord,
    SealedCheckpointRecord,
};

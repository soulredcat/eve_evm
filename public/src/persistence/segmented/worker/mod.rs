// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod append_segmented_record;
mod finish_segmented_worker;
mod observe_segmented_worker;
mod persist_segmented_batch;
mod release_segmented_scratch;
mod required_segment_scratch;
mod reserve_scratch;
mod run_segmented_worker;
mod scratch_drop_adapter;
mod start_segmented_worker;
mod try_receive_segmented_ack;
mod try_submit_segmented_batch;
mod verify_startup_marker_membership;
pub use finish_segmented_worker::finish_segmented_worker;
pub use observe_segmented_worker::observe_segmented_worker;
pub(super) use required_segment_scratch::required_segment_scratch;
pub use start_segmented_worker::start_segmented_worker;
pub use try_receive_segmented_ack::try_receive_segmented_ack;
pub use try_submit_segmented_batch::try_submit_segmented_batch;

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod append_record_request;
mod finish_record_worker;
mod observe_record_worker;
mod receive_record_ack;
mod required_record_scratch;
mod run_record_worker;
mod start_record_worker;
mod try_receive_record_ack;
mod try_submit_record;
mod types;
mod validate_worker_resources;

pub use finish_record_worker::finish_record_worker;
pub use observe_record_worker::observe_record_worker;
pub use receive_record_ack::receive_record_ack;
pub use start_record_worker::start_record_worker;
pub use try_receive_record_ack::try_receive_record_ack;
pub use try_submit_record::try_submit_record;
pub use types::{
    RecordTicket, RecordWorker, RecordWorkerError, RecordWorkerObservation, RejectedRecord,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod install_record_append_pause;
#[cfg(test)]
pub(crate) use install_record_append_pause::install_record_append_pause;

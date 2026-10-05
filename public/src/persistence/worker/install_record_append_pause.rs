// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecordWorker, tests::AppendPause};
use std::sync::mpsc::{self, Receiver, SyncSender};

/// Pause this task's worker before its next actual repository append.
pub(crate) fn install_record_append_pause(worker: &RecordWorker) -> (Receiver<()>, SyncSender<()>) {
    let (entered, entered_receiver) = mpsc::sync_channel(1);
    let (resume_sender, resume) = mpsc::sync_channel(1);
    *worker.state.pause.lock().unwrap() = Some(AppendPause { entered, resume });
    (entered_receiver, resume_sender)
}

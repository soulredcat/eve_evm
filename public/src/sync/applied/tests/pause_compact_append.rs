// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::persistence::worker::install_record_append_pause;
use crate::sync::applied::{AppliedOwner, admission::compact_worker};
use std::sync::mpsc::{Receiver, SyncSender};

pub(crate) fn pause_compact_append(owner: &AppliedOwner) -> (Receiver<()>, SyncSender<()>) {
    install_record_append_pause(compact_worker(owner).expect("compact application test fixture"))
}

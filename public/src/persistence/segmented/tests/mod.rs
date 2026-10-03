// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod fixtures;
mod limits;
mod persistence;
mod positions;
mod reservations;
mod startup_membership;
mod tails;
mod worker_lifetime;
use super::types::WorkerState;
use std::sync::{
    Arc,
    mpsc::{self, Receiver, SyncSender},
};

pub(super) struct Pause {
    pub(super) index: usize,
    pub(super) entered: SyncSender<()>,
    pub(super) resume: Receiver<()>,
    pub(super) panic: bool,
}
pub(super) fn pause(
    worker: &super::SegmentedWorker,
    index: usize,
    panic: bool,
) -> (Receiver<()>, SyncSender<()>) {
    let (entered, receiver) = mpsc::sync_channel(1);
    let (sender, resume) = mpsc::sync_channel(1);
    *worker.state.pause.lock().unwrap() = Some(Pause {
        index,
        entered,
        resume,
        panic,
    });
    (receiver, sender)
}
pub(super) fn before_record(state: &Arc<WorkerState>, index: usize) {
    let pause = {
        let mut guard = state.pause.lock().unwrap();
        if guard.as_ref().is_some_and(|pause| pause.index == index) {
            guard.take()
        } else {
            None
        }
    };
    if let Some(pause) = pause {
        let _ = pause.entered.send(());
        let _ = pause.resume.recv();
        assert!(
            !pause.panic,
            "unit-only segmented worker panic after retained admission"
        );
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    SegmentedError, SegmentedTail, SegmentedWorker, SegmentedWorkerShutdown, types::BATCHES,
};
use std::sync::atomic::Ordering;
pub fn finish_segmented_worker(worker: SegmentedWorker) -> SegmentedWorkerShutdown {
    drop(worker.sender);
    let joined = worker
        .thread
        .join()
        .map_err(|_| SegmentedError::WorkerPanicked);
    let repository = if worker.state.failed.load(Ordering::Acquire) {
        Err(SegmentedError::StorageFailed)
    } else {
        joined
    };
    let admission = worker
        .state
        .admission
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut tails: [Option<SegmentedTail>; BATCHES] = std::array::from_fn(|_| None);
    for (tail, slot) in tails.iter_mut().zip(admission.slots.iter()) {
        if let Some(slot) = slot {
            *tail = Some(SegmentedTail {
                batch: slot.batch.clone(),
                last_acknowledged_physical_cursor: slot.acknowledged,
                complete_marker: slot.complete,
            });
        }
    }
    let checkpoint_tail =
        admission
            .checkpoint
            .as_ref()
            .map(|slot| super::super::checkpoints::CheckpointTail {
                record: slot.record.clone(),
                last_acknowledged_physical_cursor: slot.acknowledged,
                complete: slot.complete,
            });
    drop(admission);
    SegmentedWorkerShutdown {
        repository,
        tails,
        checkpoint_tail,
    }
}

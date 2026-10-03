// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::PARTS;
use eve_node_policy::SegmentedRecoveryPolicy;
use eve_storage::records::segmented::SegmentedCodecLimits;
use eve_storage::records::{OpaqueRecordBudget, OpaqueRecordIdentity};
use std::{
    sync::{Arc, Mutex, atomic::AtomicBool},
    time::{Duration, Instant},
};

pub struct SegmentedPartPool {
    pub(in crate::persistence::segmented) policy: SegmentedRecoveryPolicy,
    pub(in crate::persistence::segmented) codec: SegmentedCodecLimits,
    pub(in crate::persistence::segmented) repository: OpaqueRecordBudget,
    pub(in crate::persistence::segmented) namespace: OpaqueRecordIdentity,
    pub(in crate::persistence::segmented) accounting: Mutex<PartAccounting>,
    pub(in crate::persistence::segmented) worker_active: AtomicBool,
}

pub(in crate::persistence::segmented) struct PartAccounting {
    pub(in crate::persistence::segmented) next_id: u64,
    pub(in crate::persistence::segmented) bytes: u64,
    pub(in crate::persistence::segmented) metadata: u64,
    pub(in crate::persistence::segmented) slots: [Option<PartSlot>; PARTS],
}

#[derive(Clone, Copy)]
pub(in crate::persistence::segmented) struct PartSlot {
    pub(in crate::persistence::segmented) id: u64,
    pub(in crate::persistence::segmented) bytes: u64,
    pub(in crate::persistence::segmented) created: Instant,
}
pub(in crate::persistence::segmented) struct PartLease {
    pub(in crate::persistence::segmented) pool: Arc<SegmentedPartPool>,
    pub(in crate::persistence::segmented) slot: usize,
    pub(in crate::persistence::segmented) id: u64,
}
pub(in crate::persistence::segmented) struct MetadataLease {
    pub(in crate::persistence::segmented) pool: Arc<SegmentedPartPool>,
    pub(in crate::persistence::segmented) bytes: u64,
}

pub(in crate::persistence::segmented) struct WorkerLifetimeLease {
    pub(in crate::persistence::segmented) pool: Arc<SegmentedPartPool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedPartObservation {
    pub retained_parts: usize,
    pub retained_encoded_bytes: u64,
    pub estimated_metadata_bytes: u64,
    pub oldest_age: Duration,
}

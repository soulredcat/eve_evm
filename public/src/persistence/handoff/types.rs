// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_node_policy::PublicBudget;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Resource accounting only; this pool grants no finality or durable acknowledgement.
pub struct HandoffPool {
    pub(super) budget: PublicBudget,
    pub(super) accounting: Mutex<Accounting>,
}

pub(super) struct Accounting {
    pub(super) bytes: u64,
    pub(super) next_id: u64,
    pub(super) leases: BTreeMap<u64, Instant>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandoffError {
    InvalidBudget,
    InvalidLength,
    PayloadLimit,
    QueueLimit,
    AccountingUnavailable,
    AllocationFailed,
    UnexpectedCapacity,
    IncompletePayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HandoffObservation {
    pub retained_bytes: u64,
    pub retained_batches: u64,
    pub oldest_age: Duration,
}

pub(super) struct ReservationLease {
    pub(super) pool: Arc<HandoffPool>,
    pub(super) id: u64,
    pub(super) bytes: u64,
}

/// Fixed-capacity construction buffer. Dropping an unfinished payload cancels its lease.
pub struct PayloadReservation {
    pub(super) bytes: Vec<u8>,
    pub(super) lease: ReservationLease,
}

pub(super) struct RetainedPayload {
    pub(super) bytes: Vec<u8>,
    pub(super) _lease: ReservationLease,
}

/// Immutable resource-bounded bytes, not authenticated consensus or execution input.
/// Clones retain the same charge until the final payload owner drops.
#[derive(Clone)]
pub struct RecoveryPayload(pub(super) Arc<RetainedPayload>);

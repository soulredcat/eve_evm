// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256, Bytes};
use eve_evm::{TransactionAdmission, ValidatedTransaction};
use eve_state::StateCommit;
use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, oneshot};

#[derive(Clone, Copy, Debug)]
pub struct MempoolLimits {
    pub maximum_transactions: usize,
    pub maximum_bytes: usize,
    pub maximum_per_sender: usize,
    pub maximum_nonce_gap: u64,
    pub ttl: Duration,
}
#[derive(Clone, Debug)]
pub struct PoolEntry {
    pub raw: Bytes,
    pub validated: ValidatedTransaction,
    pub admission: TransactionAdmission,
    pub admitted_at: Instant,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolError(pub String);
pub(crate) struct PoolState {
    pub head: Arc<StateCommit>,
    pub limits: MempoolLimits,
    pub bytes: usize,
    pub entries: BTreeMap<Address, BTreeMap<u64, PoolEntry>>,
    pub hashes: BTreeMap<B256, (Address, u64)>,
}
pub(crate) enum PoolCommand {
    Capture(oneshot::Sender<PoolCapture>),
    Admit(
        Bytes,
        Box<ValidatedTransaction>,
        oneshot::Sender<Result<B256, PoolError>>,
    ),
    Select(oneshot::Sender<Result<Vec<PoolEntry>, PoolError>>),
    PendingNonce(Address, oneshot::Sender<u64>),
    Find(B256, oneshot::Sender<Option<PoolEntry>>),
    Committed(Arc<StateCommit>, oneshot::Sender<Result<(), PoolError>>),
    Evict(Instant, oneshot::Sender<usize>),
}
pub(crate) type PoolCapture = (Arc<StateCommit>, Result<Vec<PoolEntry>, PoolError>);
#[derive(Clone)]
pub struct MempoolHandle {
    pub(crate) commands: mpsc::Sender<PoolCommand>,
}

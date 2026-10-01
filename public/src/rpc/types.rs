// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::mempool::MempoolHandle;
use eve_state::{StateBudget, StateCommit};
use eve_storage::state::{HistoryReadBudget, StateReader, StateService};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, atomic::AtomicBool},
};
use tokio::sync::{Semaphore, broadcast};
#[derive(Debug)]
pub(crate) struct RpcEvent {
    pub head: Value,
    pub logs: Vec<Value>,
    pub fault: Option<&'static str>,
    pub _bytes: Option<tokio::sync::OwnedSemaphorePermit>,
}
pub(crate) struct RpcContext {
    pub zone: eve_node_policy::ZoneId,
    pub producer_bytes: tokio::sync::OwnedSemaphorePermit,
    pub service: StateService,
    pub reader: StateReader,
    pub pool: MempoolHandle,
    pub state_budget: StateBudget,
    pub history_budget: HistoryReadBudget,
    pub active: Arc<Semaphore>,
    pub signatures: Arc<Semaphore>,
    pub simulations: Arc<Semaphore>,
    pub histories: Arc<Semaphore>,
    pub proofs: Arc<Semaphore>,
    pub bytes: Arc<Semaphore>,
    pub subscriptions: Arc<Semaphore>,
    pub events: broadcast::Sender<Arc<RpcEvent>>,
    pub historical_cache: Mutex<BTreeMap<u64, Arc<StateCommit>>>,
    pub healthy: AtomicBool,
}

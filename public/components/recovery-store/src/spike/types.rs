// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Serialize;

use crate::recovery::types::StorageBudget;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct StorageSpikeConfig {
    pub batch_count: usize,
    pub records_per_batch: usize,
    pub payload_bytes: usize,
    pub max_total_payload_bytes: usize,
    pub budget: StorageBudget,
}

#[derive(Clone, Debug, Serialize)]
pub struct StorageSpikeReport {
    pub profile: &'static str,
    pub batch_count: usize,
    pub records_per_batch: usize,
    pub payload_bytes: usize,
    pub budget: StorageBudget,
    pub logical_payload_bytes: u64,
    pub sync_batch_latency_us: Vec<u64>,
    pub checkpoint_latency_us: u64,
    pub database_bytes: u64,
    pub checkpoint_bytes: u64,
    pub durability_mode: &'static str,
    pub finality_verified: bool,
    pub power_loss_tested: bool,
}

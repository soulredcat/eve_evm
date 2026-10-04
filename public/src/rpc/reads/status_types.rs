// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[derive(serde::Serialize)]
pub(crate) struct NodeStatus {
    pub applied_height: u64,
    pub durable_height: u64,
    pub authenticated_height: Option<u64>,
    pub finalized_height: Option<u64>,
    pub verification_mode: &'static str,
    pub authenticated_finality: bool,
    pub ready: bool,
    pub peer_count: Option<u32>,
    pub lag: Option<u64>,
    pub database_sequence: Option<u64>,
    pub zone_id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_reason: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_lag: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated_snapshot_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_retained_height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_failed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admitted_record_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_record_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_acknowledged_physical_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_recovery_from: Option<u64>,
}

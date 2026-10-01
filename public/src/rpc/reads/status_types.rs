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
    pub peer_count: u32,
    pub lag: u64,
    pub database_sequence: u64,
    pub zone_id: u16,
}

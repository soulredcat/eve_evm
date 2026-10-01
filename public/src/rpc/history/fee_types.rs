// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FeeHistory {
    pub oldest_block: serde_json::Value,
    pub base_fee_per_gas: Vec<serde_json::Value>,
    pub gas_used_ratio: Vec<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<Vec<Vec<serde_json::Value>>>,
}

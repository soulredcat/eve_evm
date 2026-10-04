// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::B256;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct StateRootsResponse {
    pub height: Value,
    pub evm_root: B256,
    pub system_root: B256,
    pub execution_hash: B256,
    pub content_digest: B256,
    pub application_commitment: Option<B256>,
    pub verification_mode: &'static str,
    pub authenticated_finality: bool,
}

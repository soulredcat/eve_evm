// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[derive(serde::Serialize)]
pub(crate) struct RuntimeStarted {
    pub http_address: String,
    pub ws_address: String,
    pub height: u64,
    pub verification_mode: &'static str,
    pub authenticated_finality: bool,
    pub zone_id: u16,
}

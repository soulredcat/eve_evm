// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub(in crate::consensus::runtime) struct SignerPosition {
    pub height: i64,
    pub round: i32,
    pub step: i32,
}

#[derive(Serialize)]
pub(in crate::consensus::runtime) struct NodeFailureRecord {
    pub version: u8,
    pub process_id: u32,
    pub categories: Vec<&'static str>,
    pub io_kind: Option<String>,
    pub application_height: Option<i64>,
    pub signer_last: Option<SignerPosition>,
    pub signer_requested: Option<SignerPosition>,
}

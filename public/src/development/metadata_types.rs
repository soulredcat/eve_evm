// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[derive(serde::Serialize)]
pub(crate) struct PublicNodeMetadata<'a> {
    pub schema: &'static str,
    pub zone_id: u16,
    pub genesis: alloy_primitives::B256,
    pub chain_id: u64,
    pub network_name: &'a str,
    pub verification_mode: &'static str,
    pub authenticated_finality: bool,
}

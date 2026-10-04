// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::policy_types::AdapterReview;
use serde::Deserialize;

/// Adapter declarations only; fragments cannot grant exclusions or size waivers.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterPolicyFragment {
    pub version: u32,
    pub adapters: Vec<AdapterReview>,
}

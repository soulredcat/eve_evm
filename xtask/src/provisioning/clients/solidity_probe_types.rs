// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct SoliditySource {
    pub content: &'static str,
}

#[derive(Serialize)]
pub struct SoliditySettings {
    #[serde(rename = "evmVersion")]
    pub evm_version: &'static str,
    #[serde(rename = "outputSelection")]
    pub output_selection: BTreeMap<&'static str, BTreeMap<&'static str, &'static [&'static str]>>,
}

#[derive(Serialize)]
pub struct SolidityProbeInput {
    pub language: &'static str,
    pub sources: BTreeMap<&'static str, SoliditySource>,
    pub settings: SoliditySettings,
}

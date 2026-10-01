// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::engine::{
    types::{EngineHomeMarker, MARKER_NAME},
    verification::read_engine_file,
};
use anyhow::{Result, ensure};
use std::path::Path;

pub(in crate::development::engine) fn read_engine_home_marker(
    home: &Path,
) -> Result<EngineHomeMarker> {
    let marker: EngineHomeMarker =
        serde_json::from_slice(&read_engine_file(&home.join(MARKER_NAME), 4096)?)?;
    ensure!(
        marker.schema == 1 && marker.stage == "ready",
        "engine home is unrecognized/incomplete; never reset"
    );
    Ok(marker)
}

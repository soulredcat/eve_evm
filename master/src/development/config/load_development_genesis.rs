// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::read_bounded_input::read_bounded_input;
use anyhow::Result;
use eve_protocol_config::genesis::{DevelopmentGenesis, input::decode_development_spec};
use std::path::Path;

pub fn load_development_genesis(path: &Path) -> Result<DevelopmentGenesis> {
    decode_development_spec(&read_bounded_input(path, 1_048_576)?)
        .map_err(|error| anyhow::anyhow!("invalid development genesis: {error:?}"))
}

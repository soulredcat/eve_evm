// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use anyhow::{Result, ensure};
use eve_protocol_config::genesis::DevelopmentGenesis;
use std::{path::Path, sync::Arc};

pub(crate) fn load_acceptance_fixture(
    path: Option<&Path>,
    genesis: &DevelopmentGenesis,
) -> Result<Option<Arc<AcceptanceFixture>>> {
    super::super::validate_acceptance_profile::validate_acceptance_profile(genesis, false)?;
    ensure!(
        path.is_none(),
        "acceptance fixture is absent from normal runtime builds"
    );
    Ok(None)
}

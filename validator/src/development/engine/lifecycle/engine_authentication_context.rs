// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::engine::{OwnedEngine, VerifiedEngineImage};
use anyhow::Result;
use std::process::Child;

pub(crate) fn engine_authentication_context(
    engine: &mut OwnedEngine,
) -> Result<(&mut Child, &VerifiedEngineImage)> {
    super::engine_child(engine)?;
    let child = engine
        .child
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("owned engine has stopped"))?;
    Ok((child, &engine.image))
}

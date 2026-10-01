// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::EngineLease;

/// Explicit unlock releases inherited open-file-description locks before closing this handle.
pub(in crate::development::engine) fn release_engine_lease(
    lease: &mut EngineLease,
) -> std::io::Result<()> {
    if lease.locked {
        lease.file.unlock()?;
        lease.locked = false;
    }
    Ok(())
}

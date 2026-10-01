// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// The EVE fee journal allocates collected fees once after block execution.
pub(crate) fn suppress_proposer_reward<Error>() -> Result<(), Error> {
    Ok(())
}

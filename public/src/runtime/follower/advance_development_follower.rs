// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::advance_follower_iteration::advance_follower_iteration;
use crate::sync::applied::AppliedOwner;
use crate::sync::follower::ValidatorFollowerSource;
use anyhow::Result;

/// Return the exclusive owner even when an ordinary iteration fails.
pub(super) fn advance_development_follower(
    mut owner: AppliedOwner,
    source: ValidatorFollowerSource,
) -> (AppliedOwner, Result<bool>) {
    let result = advance_follower_iteration(&mut owner, source);
    (owner, result)
}

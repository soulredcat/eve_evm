// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AuthenticatedCheckpoint;
use crate::AuthenticatedApplicationAnchor;
pub fn checkpoint_anchor(checkpoint: &AuthenticatedCheckpoint) -> &AuthenticatedApplicationAnchor {
    &checkpoint.anchor
}

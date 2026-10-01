// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateView;
use crate::StateVersion;

pub fn view_version(view: &StateView) -> &StateVersion {
    &view.version
}

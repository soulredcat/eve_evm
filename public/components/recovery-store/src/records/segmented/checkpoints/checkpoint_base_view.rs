// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointBasePreflight, CheckpointBaseView};

pub fn checkpoint_base_view<'a>(preflight: &CheckpointBasePreflight<'a>) -> CheckpointBaseView<'a> {
    preflight.view
}

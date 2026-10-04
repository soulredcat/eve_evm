// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointBaseView, inspection_types::CheckpointBaseInspection};

pub fn checkpoint_base_inspection_view<'a>(
    inspection: &CheckpointBaseInspection<'a>,
) -> CheckpointBaseView<'a> {
    inspection.view
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedMode, AppliedPublication, state::applied_state_mode};

pub fn applied_mode(publication: &AppliedPublication) -> AppliedMode {
    applied_state_mode(&publication.generation.state)
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateView;
use crate::CompleteState;

pub fn view_state(view: &StateView) -> &CompleteState {
    &view.state
}

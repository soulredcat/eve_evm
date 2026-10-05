// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery::import::{ImportedState, ImportedTransition};

pub fn into_imported_state(transition: ImportedTransition) -> Arc<ImportedState> {
    transition.state
}

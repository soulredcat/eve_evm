// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery::import::{AuthenticatedImportInput, ImportedTransition};

pub fn imported_transition_input(
    transition: &ImportedTransition,
) -> &Arc<AuthenticatedImportInput> {
    &transition.input
}

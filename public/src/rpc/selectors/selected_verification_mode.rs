// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::SelectedState;
use crate::sync::applied::{AppliedMode, applied_anchor, applied_mode};

pub(crate) fn selected_verification_mode(selected: &SelectedState) -> &'static str {
    match selected {
        SelectedState::Applied { publication } if applied_anchor(publication).is_none() => {
            "LOCAL_GENESIS_TRUSTED"
        }
        SelectedState::Applied { publication } => match applied_mode(publication) {
            AppliedMode::EmptyReplay => "INDEPENDENT_EMPTY_REPLAY",
            AppliedMode::AuthenticatedImport => "AUTHENTICATED_IMPORT",
        },
        _ => "LOCAL_DEV_UNAUTHENTICATED",
    }
}

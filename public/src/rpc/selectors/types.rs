// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;
use eve_state::StateCommit;
use eve_storage::state::ImmutableStateView;
use std::sync::Arc;
use tokio::sync::OwnedSemaphorePermit;
pub(crate) enum SelectedState {
    Applied {
        publication: Arc<AppliedPublication>,
    },
    Current {
        view: Arc<ImmutableStateView>,
        _lease: Option<OwnedSemaphorePermit>,
    },
    Owned {
        commit: Arc<StateCommit>,
        _lease: Option<OwnedSemaphorePermit>,
    },
}

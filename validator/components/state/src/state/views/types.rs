// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{CompleteState, StateVersion};
use std::sync::Arc;

/// Immutable locally consistent view. No authentication/durability is inferred.
#[derive(Clone, Debug)]
pub struct StateView {
    pub(crate) state: Arc<CompleteState>,
    pub(crate) version: StateVersion,
}

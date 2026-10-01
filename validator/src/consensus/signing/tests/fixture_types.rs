// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::SignerConfig;
use eve_storage::state::{StateRepository, StateService};
use std::{path::PathBuf, sync::Arc};

pub(in crate::consensus) struct TestFixture {
    pub root: PathBuf,
    pub store: StateRepository,
    pub service: Arc<StateService>,
    pub config: SignerConfig,
}

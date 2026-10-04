// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native::Frame;
use eve_state::{DevelopmentGenesis, StateCommit};

/// Preserved fixture executor parameter; a numeric estimate does not acquire a runtime lease.
pub const CLONE_BYTES: usize = 512 * 1_048_576;
pub struct RecoveryChain {
    pub genesis: DevelopmentGenesis,
    pub commits: Vec<StateCommit>,
    pub frames: Vec<Frame>,
}

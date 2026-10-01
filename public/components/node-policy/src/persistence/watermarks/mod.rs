// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod types;
mod validate_watermarks;

pub use types::{
    AppliedHeight, AuthenticatedStateHeight, CheckpointHeight, DurableRecoveryHeight,
    FinalizedHeight, PublicWatermarks, StateAuthentication,
};
pub use validate_watermarks::validate_watermarks;

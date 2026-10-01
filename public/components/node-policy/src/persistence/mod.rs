// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Public persistence/readiness watermarks, distinct from validator sign state.

mod readiness;
mod watermarks;

pub use readiness::{PersistenceObservation, PublicReadiness, evaluate_public_readiness};
pub use watermarks::{
    AppliedHeight, AuthenticatedStateHeight, CheckpointHeight, DurableRecoveryHeight,
    FinalizedHeight, PublicWatermarks, StateAuthentication, validate_watermarks,
};

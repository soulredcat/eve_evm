// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod binding;
mod types;
mod verification;

pub use binding::encode_hybrid_message;
pub use types::{
    AuthorizationPurpose, EnrolledHybridIdentity, HybridAuthorization, HybridSignature,
};
pub use verification::verify_hybrid_authorization;

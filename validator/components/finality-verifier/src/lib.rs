// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! EVE development history and authenticated application outcomes; not replay proof.
mod finality;

pub use finality::{
    AuthenticatedApplicationAnchor, DevelopmentFinalityVerifier, FinalityError,
    VerifiedDevelopmentHeader, authenticate_current_application_version,
    initialize_development_finality, verify_next_development_header,
};

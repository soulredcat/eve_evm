// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod evaluate_source;
mod types;

pub use evaluate_source::evaluate_source;
pub use types::{SourceEligibility, SourceObservation, SourcePurpose, SourceVerification, ZoneId};

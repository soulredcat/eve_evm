// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Genesis-bound disposable B3 lifecycle/fault fixtures, absent from normal builds.
#[cfg(feature = "development-acceptance")]
mod enabled;
#[cfg(not(feature = "development-acceptance"))]
mod unavailable;
mod validate_acceptance_profile;
#[cfg(feature = "development-acceptance")]
pub(crate) use enabled::{
    AcceptanceFixture, acceptance_key_enrolled, acceptance_validator_updates,
    extend_acceptance_proposer_owners, load_acceptance_fixture, poison_development_proposal,
    record_acceptance_rejection, validate_acceptance_proposer_owners,
};
#[cfg(not(feature = "development-acceptance"))]
pub(crate) use unavailable::{
    AcceptanceFixture, acceptance_key_enrolled, acceptance_validator_updates,
    extend_acceptance_proposer_owners, load_acceptance_fixture, poison_development_proposal,
    record_acceptance_rejection, validate_acceptance_proposer_owners,
};

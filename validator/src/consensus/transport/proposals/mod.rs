// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod binding;
mod issue_verified_finalize_proposal;
mod issue_verified_process_proposal;
mod request;
mod source;
mod types;

pub(in crate::consensus::transport) use issue_verified_finalize_proposal::issue_verified_finalize_proposal;
pub(in crate::consensus::transport) use issue_verified_process_proposal::issue_verified_process_proposal;
pub(in crate::consensus) use types::{
    EngineProposalBinding, NativeProposalSource, VerifiedLocalEngineProposal,
};

#[cfg(test)]
mod fixture_verified_local_engine_proposal;
#[cfg(test)]
pub(in crate::consensus) use fixture_verified_local_engine_proposal::fixture_verified_local_engine_proposal;

#[cfg(test)]
mod fixture_verified_finalize_proposal;
#[cfg(test)]
pub(in crate::consensus) use fixture_verified_finalize_proposal::fixture_verified_finalize_proposal;

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Private validator signing policy; durable storage is not finality or enrollment.
mod fence_signer;
mod open_durable_signer;
mod persist_signature;
mod policy;
mod records;
mod sign_proposal;
mod sign_vote;
mod signer_status;
mod types;
mod validate_signer_config;

pub(in crate::consensus) use fence_signer::fence_signer;
pub(in crate::consensus) use open_durable_signer::open_durable_signer;
pub(in crate::consensus) use policy::check_current_height;
pub(in crate::consensus) use sign_proposal::sign_proposal;
pub(in crate::consensus) use sign_vote::sign_vote;
pub(in crate::consensus) use signer_status::signer_status;
pub(in crate::consensus) use types::{DurableSigner, SignerConfig, SignerStatus};
pub(in crate::consensus) use validate_signer_config::validate_signer_config;

#[cfg(test)]
pub(in crate::consensus) mod tests;

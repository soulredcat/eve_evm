// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Actual local-engine application transitions; durability never grants finality.
mod admission;
mod committing;
mod context;
mod finalizing;
mod initialization;
mod opening;
mod preparing;
mod processing;
mod reading;
mod replay;
mod safety;
mod types;

pub(in crate::consensus) use admission::check_transaction;
pub(in crate::consensus) use committing::commit_application;
pub(in crate::consensus) use context::application_finalization_binding;
pub(in crate::consensus) use context::application_proposal_binding;
pub(in crate::consensus) use finalizing::finalize_block;
pub(in crate::consensus) use initialization::initialize_application;
pub(in crate::consensus) use opening::open_application;
pub(in crate::consensus) use preparing::prepare_proposal;
pub(in crate::consensus) use processing::{application_approval_registry, process_proposal};
pub(in crate::consensus) use reading::application_info;
pub(in crate::consensus) use types::{ApplicationConfig, ConsensusApplication};

#[cfg(test)]
mod tests;

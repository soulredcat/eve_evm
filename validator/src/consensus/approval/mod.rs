// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Actual execution over an authenticated local-engine request, not a public header proof.
mod approval_error_diagnostic;
mod approval_request;
mod approved_state_block;
mod check_approval_parent;
mod check_vote_approval;
mod classify_execution_error;
mod create_execution_approval;
mod registry;
mod types;

pub(in crate::consensus) use approval_error_diagnostic::approval_error_diagnostic;
pub(in crate::consensus) use approval_request::approval_request;
pub(in crate::consensus) use approved_state_block::approved_state_block;
use check_approval_parent::check_approval_parent;
pub(in crate::consensus) use check_vote_approval::check_vote_approval;
use classify_execution_error::classify_execution_error;
pub(in crate::consensus) use create_execution_approval::create_execution_approval;
pub(in crate::consensus) use registry::{
    ApprovalRegistry, ApprovalRegistryStatus, approval_registry_status, clear_after_synced_commit,
    create_approval_registry, find_approval, pin_signed_vote_approval, publish_native_approval,
    reconstruct_retained_approval,
};
pub(in crate::consensus) use types::{ApprovalError, ExecutionApproval};

#[cfg(test)]
mod tests;

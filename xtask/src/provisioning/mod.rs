// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Task-local, digest-pinned development tools. No chain or release authority.
mod artifacts;
mod builds;
mod clients;
mod execution;
mod orchestration;
mod paths;
mod pins;
mod receipts;
mod types;

pub(crate) use artifacts::{
    compute_artifact_digest, read_archive_listing, validate_archive_links, validate_archive_members,
};
pub use orchestration::{provision_b0_tools, validate_provisioned_tools};
pub(crate) use paths::resolve_contained_path;
pub use types::{ProvisionedArtifact, ProvisionedTools};

#[cfg(test)]
#[path = "../../tests/provisioning_checks/mod.rs"]
mod tests;

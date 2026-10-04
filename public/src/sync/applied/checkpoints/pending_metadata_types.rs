// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Local repair observation; matching metadata and marker presence grant no authenticated authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointMetadataRepairStatus {
    Missing,
    MatchingValid,
    RemovedInvalid,
    PublishedCompletionPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointMetadataRepairOutcome {
    pub manifest: CheckpointMetadataRepairStatus,
    pub completion: CheckpointMetadataRepairStatus,
}

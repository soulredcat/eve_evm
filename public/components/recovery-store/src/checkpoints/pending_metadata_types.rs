// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointPendingMetadataKind {
    Manifest,
    Completion,
}

/// Read-only local metadata observations; marker presence grants no verified completion authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointPendingMetadataStatus {
    Missing,
    MatchingValid,
    InvalidKnown,
    Refused,
    PublishedCompletionPresent,
}

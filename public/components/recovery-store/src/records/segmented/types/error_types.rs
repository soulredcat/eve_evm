// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmentedCodecError {
    InvalidLimits,
    LimitExceeded,
    MalformedEncoding,
    UnsupportedVersion,
    UnsupportedMode,
    InvalidIdentity,
    InvalidChunk,
    InvalidReferences,
    HashMismatch,
    OutputCapacity,
    AllocationFailed,
}

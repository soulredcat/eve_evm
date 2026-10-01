// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Earned only by bounded full-file hashing; the held descriptor preserves the verified inode.
pub(crate) struct VerifiedEngineImage {
    pub(super) file: std::fs::File,
    pub(super) digest: [u8; 32],
    pub(super) identity: EngineImageIdentity,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct EngineImageIdentity {
    pub(super) device: u64,
    pub(super) inode: u64,
    pub(super) length: u64,
    pub(super) modified_seconds: i64,
    pub(super) modified_nanos: i64,
    pub(super) changed_seconds: i64,
    pub(super) changed_nanos: i64,
}

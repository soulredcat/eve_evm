// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointBaseView;

/// Sealed local framing over the same immutable bytes. Metadata and canonical
/// target fields are untrusted until separate semantic and proof verification.
#[derive(Debug)]
pub struct CheckpointBaseInspection<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) view: CheckpointBaseView<'a>,
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::SegmentedCodecError;

pub(in crate::records::segmented) fn read_segmented_array<const N: usize>(
    bytes: &[u8],
    offset: usize,
) -> Result<[u8; N], SegmentedCodecError> {
    let end = offset
        .checked_add(N)
        .ok_or(SegmentedCodecError::MalformedEncoding)?;
    bytes
        .get(offset..end)
        .ok_or(SegmentedCodecError::MalformedEncoding)?
        .try_into()
        .map_err(|_| SegmentedCodecError::MalformedEncoding)
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Mutates one existing framed field, preserving every other exact source byte.
pub fn replace_field(
    bytes: &[u8],
    prefix: usize,
    field: usize,
    replace: impl FnOnce(&[u8]) -> Vec<u8>,
) -> Vec<u8> {
    let mut offset = prefix;
    for _ in 0..field {
        let size = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4 + size;
    }
    let size = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
    let changed = replace(&bytes[offset + 4..offset + 4 + size]);
    let mut output = bytes[..offset].to_vec();
    output.extend_from_slice(&u32::try_from(changed.len()).unwrap().to_be_bytes());
    output.extend_from_slice(&changed);
    output.extend_from_slice(&bytes[offset + 4 + size..]);
    output
}

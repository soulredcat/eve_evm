// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Bytes, keccak256};

/// Minimal bytecode produces genuine canonical EVM receipts for independent event-authentication negatives.
pub(in crate::development::acceptance::enabled::tests) fn emitter_code(
    digest: &[u8; 32],
    footer: &[u8],
    count: usize,
) -> Bytes {
    let mut code = Vec::new();
    for _ in 0..count {
        code.extend_from_slice(&[0x60, 0x01, 0x7f]); // action topic, PUSH32 digest
        code.extend_from_slice(digest);
        code.push(0x7f);
        code.extend_from_slice(keccak256(b"Transition(bytes32,uint8)").as_slice());
        code.extend_from_slice(&[0x60, 0x00, 0x60, 0x00, 0xa3]); // empty memory and LOG3
    }
    code.push(0x00);
    code.extend_from_slice(footer);
    code.into()
}

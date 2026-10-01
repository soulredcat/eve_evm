// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};

pub(super) fn decode_acceptance_key(input: &str) -> Result<[u8; 32]> {
    ensure!(
        input.len() == 64 && input.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "invalid acceptance enrollment key encoding"
    );
    let key = hex::decode(input)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("acceptance key width"))?;
    eve_protocol_config::genesis::validate_classical_enrollment_key(&key)
        .map_err(|_| anyhow::anyhow!("noncanonical or non-prime-order acceptance key rejected"))?;
    Ok(key)
}

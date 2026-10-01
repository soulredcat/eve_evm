// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::KeyPossessionInput;
use crate::{encoding::encode_list, network::validate_security_profile};

/// Classical-dev possession message; B5 must verify the signature, recovered
/// owner, role, nonce and registry epoch before any custody/lifecycle change.
pub fn encode_key_possession(input: KeyPossessionInput) -> Result<Vec<u8>, &'static str> {
    validate_security_profile(input.profile).map_err(|_| "unsupported possession profile")?;
    if input.chain_id == 0
        || input.protocol_version == 0
        || input.owner.is_zero()
        || ![1, 2].contains(&input.role)
    {
        return Err("invalid possession domain");
    }
    Ok(encode_list(&[
        alloy_rlp::encode(b"EVE_KEY_POSSESSION_V1".as_slice()),
        alloy_rlp::encode(input.genesis.0),
        alloy_rlp::encode(input.protocol_version),
        alloy_rlp::encode(input.profile as u8),
        alloy_rlp::encode(input.chain_id),
        alloy_rlp::encode(input.owner),
        alloy_rlp::encode(input.role),
        alloy_rlp::encode(input.nonce),
        alloy_rlp::encode(input.key_epoch),
        alloy_rlp::encode(input.public_key.as_slice()),
    ]))
}

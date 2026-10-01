// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_value, take_bytes, take_list};
use crate::{StateError, StateIdentity};
use eve_protocol_config::{network::decode_security_profile, records::GenesisHash};

pub(crate) fn decode_identity(input: &mut &[u8]) -> Result<StateIdentity, StateError> {
    let mut list = take_list(input)?;
    let genesis = GenesisHash(decode_value(&mut list)?);
    let network_name = String::from_utf8(take_bytes(&mut list, 64)?.to_vec())
        .map_err(|_| StateError::MalformedEncoding)?;
    let identity = StateIdentity {
        genesis,
        network_name,
        evm_chain_id: decode_value(&mut list)?,
        protocol_version: decode_value(&mut list)?,
        security_profile: decode_security_profile(decode_value(&mut list)?)
            .map_err(|_| StateError::InvalidIdentity)?,
        key_epoch: decode_value(&mut list)?,
        configuration_digest: decode_value(&mut list)?,
    };
    if !list.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(identity)
}

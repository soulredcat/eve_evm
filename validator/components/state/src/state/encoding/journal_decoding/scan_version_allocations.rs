// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::take_borrowed_bytes;
use crate::{
    B256, StateError,
    state::encoding::{decode_value, take_list},
};
use eve_protocol_config::network::decode_security_profile;

/// Scan version field widths without allocating its network String or reencoding.
pub(crate) fn scan_version_allocations(bytes: &[u8]) -> Result<usize, StateError> {
    let mut remaining = bytes;
    let mut version = take_list(&mut remaining)?;
    let mut identity = take_list(&mut version)?;
    decode_value::<B256>(&mut identity)?;
    let network_name = take_borrowed_bytes(&mut identity, 64)?;
    core::str::from_utf8(network_name).map_err(|_| StateError::MalformedEncoding)?;
    decode_value::<u64>(&mut identity)?;
    decode_value::<u32>(&mut identity)?;
    decode_security_profile(decode_value::<u8>(&mut identity)?)
        .map_err(|_| StateError::InvalidIdentity)?;
    decode_value::<u64>(&mut identity)?;
    decode_value::<B256>(&mut identity)?;
    decode_value::<u64>(&mut version)?;
    decode_value::<u64>(&mut version)?;
    decode_value::<B256>(&mut version)?;
    decode_value::<B256>(&mut version)?;
    decode_value::<B256>(&mut version)?;
    let mut optional = take_list(&mut version)?;
    match decode_value::<u8>(&mut optional)? {
        0 => {}
        1 => {
            decode_value::<B256>(&mut optional)?;
        }
        _ => return Err(StateError::MalformedEncoding),
    }
    decode_value::<B256>(&mut version)?;
    if !identity.is_empty() || !optional.is_empty() || !version.is_empty() || !remaining.is_empty()
    {
        return Err(StateError::MalformedEncoding);
    }
    Ok(network_name.len())
}

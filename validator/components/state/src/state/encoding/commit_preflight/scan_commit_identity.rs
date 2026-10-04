// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    B256, StateError,
    state::encoding::{decode_value, journal_decoding::take_borrowed_bytes, take_list},
};
use eve_protocol_config::network::decode_security_profile;

pub(super) fn scan_commit_identity(input: &mut &[u8]) -> Result<usize, StateError> {
    let mut identity = take_list(input)?;
    decode_value::<B256>(&mut identity)?;
    let network = take_borrowed_bytes(&mut identity, 64)?;
    core::str::from_utf8(network).map_err(|_| StateError::MalformedEncoding)?;
    decode_value::<u64>(&mut identity)?;
    decode_value::<u32>(&mut identity)?;
    decode_security_profile(decode_value::<u8>(&mut identity)?)
        .map_err(|_| StateError::InvalidIdentity)?;
    decode_value::<u64>(&mut identity)?;
    decode_value::<B256>(&mut identity)?;
    if !identity.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(network.len())
}

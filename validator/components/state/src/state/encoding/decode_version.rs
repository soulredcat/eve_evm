// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_identity, decode_value, take_list};
use crate::{StateError, StateVersion};
use eve_protocol_config::records::{
    ApplicationCommitment, EvmStateRoot, ExecutionBlockHash, SystemStateRoot,
};

pub(crate) fn decode_version(input: &mut &[u8]) -> Result<StateVersion, StateError> {
    let mut list = take_list(input)?;
    let identity = decode_identity(&mut list)?;
    let height = decode_value(&mut list)?;
    let timestamp = decode_value(&mut list)?;
    let execution_hash = ExecutionBlockHash(decode_value(&mut list)?);
    let evm_root = EvmStateRoot(decode_value(&mut list)?);
    let system_root = SystemStateRoot(decode_value(&mut list)?);
    let mut optional = take_list(&mut list)?;
    let application = match decode_value::<u8>(&mut optional)? {
        0 => None,
        1 => Some(ApplicationCommitment(decode_value(&mut optional)?)),
        _ => return Err(StateError::MalformedEncoding),
    };
    let content_digest = decode_value(&mut list)?;
    if !optional.is_empty() || !list.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(StateVersion {
        identity,
        height,
        timestamp,
        execution_hash,
        evm_root,
        system_root,
        application,
        content_digest,
    })
}

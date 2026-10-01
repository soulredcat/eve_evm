// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{NetworkProfileBinding, ProfileError, validate_security_profile};

pub fn validate_network_profile(
    expected: &NetworkProfileBinding,
    received: &NetworkProfileBinding,
) -> Result<(), ProfileError> {
    if expected.genesis_hash.is_zero()
        || expected.network_name.is_empty()
        || expected.network_name.len() > 64
        || expected.evm_chain_id == 0
        || expected.protocol_version == 0
        || expected.activation_height == 0
    {
        return Err(ProfileError::InvalidBinding);
    }
    if expected.genesis_hash != received.genesis_hash
        || expected.network_name != received.network_name
        || expected.evm_chain_id != received.evm_chain_id
    {
        return Err(ProfileError::WrongNetwork);
    }
    if expected.protocol_version != received.protocol_version {
        return Err(ProfileError::WrongProtocol);
    }
    if expected.profile != received.profile
        || expected.activation_height != received.activation_height
        || expected.key_epoch != received.key_epoch
    {
        return Err(ProfileError::WrongProfileHistory);
    }
    validate_security_profile(expected.profile)
}

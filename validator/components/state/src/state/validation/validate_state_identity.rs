// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::network::{NetworkProfileBinding, validate_network_profile};

use crate::{StateError, StateIdentity};

pub(crate) fn validate_state_identity(identity: &StateIdentity) -> Result<(), StateError> {
    if identity.configuration_digest.is_zero()
        || identity.network_name.is_empty()
        || identity.network_name.len() > 64
        || identity.protocol_version != 1
    {
        return Err(StateError::InvalidIdentity);
    }
    let profile = NetworkProfileBinding {
        genesis_hash: identity.genesis.0,
        network_name: identity.network_name.clone(),
        evm_chain_id: identity.evm_chain_id,
        protocol_version: identity.protocol_version,
        profile: identity.security_profile,
        activation_height: 1,
        key_epoch: identity.key_epoch,
    };
    validate_network_profile(&profile, &profile).map_err(|_| StateError::InvalidIdentity)
}

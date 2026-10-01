// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_list;
use crate::StateIdentity;

pub(crate) fn encode_identity(identity: &StateIdentity) -> Vec<u8> {
    encode_list(&[
        alloy_rlp::encode(identity.genesis.0),
        alloy_rlp::encode(identity.network_name.as_bytes()),
        alloy_rlp::encode(identity.evm_chain_id),
        alloy_rlp::encode(identity.protocol_version),
        alloy_rlp::encode(identity.security_profile as u8),
        alloy_rlp::encode(identity.key_epoch),
        alloy_rlp::encode(identity.configuration_digest),
    ])
}

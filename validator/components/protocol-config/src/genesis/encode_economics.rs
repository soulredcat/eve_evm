// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::encoding::encode_list;

use super::EconomicsParameters;

pub(crate) fn encode_economics(value: &EconomicsParameters) -> Vec<u8> {
    encode_list(&[
        alloy_rlp::encode(value.burn_bps),
        alloy_rlp::encode(value.node_pool_bps),
        alloy_rlp::encode(value.validator_pool_bps),
        alloy_rlp::encode(value.epoch_blocks),
        alloy_rlp::encode(value.maximum_validators),
        alloy_rlp::encode(value.validator_self_bond),
        alloy_rlp::encode(value.node_self_bond),
        alloy_rlp::encode(value.default_commission_bps),
        alloy_rlp::encode(value.maximum_commission_bps),
        alloy_rlp::encode(value.unbonding_seconds),
        alloy_rlp::encode(value.unbonding_blocks),
        alloy_rlp::encode(value.maximum_tasks_per_block),
        alloy_rlp::encode(value.node_availability_bps),
        alloy_rlp::encode(value.double_sign_slash_bps),
        alloy_rlp::encode(value.downtime_window_blocks),
        alloy_rlp::encode(value.minimum_participation_bps),
        alloy_rlp::encode(u8::from(value.node_service_slashing)),
        alloy_rlp::encode(value.default_issuance),
    ])
}

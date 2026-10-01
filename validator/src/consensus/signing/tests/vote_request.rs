// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::SignerConfig;
use eve_consensus_comet::{
    consensus::certificates::validator_address, wire::tendermint::types::Vote,
};

pub(in crate::consensus) fn vote_request(config: &SignerConfig) -> Vote {
    Vote {
        r#type: 1,
        height: 1,
        round: 0,
        validator_address: validator_address(&config.expected_public_key).to_vec(),
        validator_index: 0,
        timestamp: Some(prost_types::Timestamp {
            seconds: 1_728_000_001,
            nanos: 123,
        }),
        ..Default::default()
    }
}

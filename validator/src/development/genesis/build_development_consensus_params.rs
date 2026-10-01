// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::tendermint::types::{
    AbciParams, AuthorityParams, BlockParams, ConsensusParams, EvidenceParams, ValidatorParams,
    VersionParams,
};

/// Classical, bounded development policy; no hybrid or production activation.
pub(crate) fn build_development_consensus_params(protocol_version: u32) -> ConsensusParams {
    let limits = eve_protocol_config::genesis::development_consensus_limits();
    ConsensusParams {
        block: Some(BlockParams {
            max_bytes: limits.maximum_block_bytes as i64,
            max_gas: limits.block_gas_limit as i64,
        }),
        evidence: Some(EvidenceParams {
            max_age_num_blocks: limits.evidence_max_age_blocks as i64,
            max_age_duration: Some(prost_types::Duration {
                seconds: limits.evidence_max_age_seconds as i64,
                nanos: 0,
            }),
            max_bytes: 1_048_576,
        }),
        validator: Some(ValidatorParams {
            pub_key_types: vec!["ed25519".into()],
        }),
        version: Some(VersionParams {
            app: u64::from(protocol_version),
        }),
        abci: Some(AbciParams {
            vote_extensions_enable_height: 0,
        }),
        authority: Some(AuthorityParams {
            authority: String::new(),
        }),
    }
}

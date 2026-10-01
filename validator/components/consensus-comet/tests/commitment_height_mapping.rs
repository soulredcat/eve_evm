// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::{
    consensus::commitments::{
        ExecutionHeight, HeightMappingError, map_finalize_height, match_next_header_commitment,
    },
    wire::tendermint::types::Header,
};

#[test]
fn maps_post_state_and_validator_updates_at_distinct_heights() {
    for height in [1, 42, i64::MAX - 3] {
        let mapping = map_finalize_height(ExecutionHeight(height)).unwrap();
        assert_eq!(mapping.app_hash_header_height, height + 1);
        assert_eq!(mapping.update_next_validators_hash_height, height + 1);
        assert_eq!(mapping.updated_validator_set_height, height + 2);
        assert_eq!(mapping.updated_last_commit_metadata_height, height + 3);
    }
}

#[test]
fn rejects_nonpositive_or_unrepresentable_mapping() {
    for height in [0, -1, i64::MIN] {
        assert_eq!(
            map_finalize_height(ExecutionHeight(height)),
            Err(HeightMappingError::NonPositiveExecutionHeight)
        );
    }
    for height in [i64::MAX - 2, i64::MAX - 1, i64::MAX] {
        assert_eq!(
            map_finalize_height(ExecutionHeight(height)),
            Err(HeightMappingError::HeightOverflow)
        );
    }
}

#[test]
fn matches_only_next_height_same_chain_exact_commitment() {
    let commitment = [0x42; 32];
    let mut header = Header {
        chain_id: "eve-local-v1".into(),
        height: 43,
        app_hash: commitment.to_vec(),
        ..Default::default()
    };
    let matched =
        match_next_header_commitment("eve-local-v1", ExecutionHeight(42), &commitment, &header)
            .unwrap();
    assert_eq!(matched.consensus_header_height, 43);
    for height in [41, 42, 44] {
        header.height = height;
        assert_eq!(
            match_next_header_commitment("eve-local-v1", ExecutionHeight(42), &commitment, &header),
            Err(HeightMappingError::WrongHeaderHeight)
        );
    }
    header.height = 43;
    header.chain_id = "attacker-network".into();
    assert_eq!(
        match_next_header_commitment("eve-local-v1", ExecutionHeight(42), &commitment, &header),
        Err(HeightMappingError::WrongChain)
    );
    header.chain_id = "eve-local-v1".into();
    for hash in [vec![0x43; 32], vec![0x42; 31], vec![0x42; 33], vec![]] {
        header.app_hash = hash;
        assert_eq!(
            match_next_header_commitment("eve-local-v1", ExecutionHeight(42), &commitment, &header),
            Err(HeightMappingError::WrongApplicationHash)
        );
    }
}

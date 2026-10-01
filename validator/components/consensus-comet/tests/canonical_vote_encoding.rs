// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::tendermint::types::CanonicalVote;
use prost::Message;

/// Ported byte vectors from the pinned engine's TestVoteSignBytesTestVectors.
/// Source: https://github.com/cometbft/cometbft/blob/0880b4d378f347ab16e54ec677ff50d803f37d62/types/vote_test.go
#[test]
fn canonical_vote_bytes_match_pinned_upstream_vectors() {
    let timestamp = Some(prost_types::Timestamp {
        seconds: -62_135_596_800,
        nanos: 0,
    });
    for (message_type, chain, expected) in [
        (
            2,
            "",
            "2108021101000000000000001901000000000000002a0b088092b8c398feffffff01",
        ),
        (
            1,
            "",
            "2108011101000000000000001901000000000000002a0b088092b8c398feffffff01",
        ),
        (
            0,
            "test_chain_id",
            "2e1101000000000000001901000000000000002a0b088092b8c398feffffff01320d746573745f636861696e5f6964",
        ),
    ] {
        let vote = CanonicalVote {
            r#type: message_type,
            height: 1,
            round: 1,
            block_id: None,
            timestamp,
            chain_id: chain.into(),
        };
        assert_eq!(hex::encode(vote.encode_length_delimited_to_vec()), expected);
    }
}

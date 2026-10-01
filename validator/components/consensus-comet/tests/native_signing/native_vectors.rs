// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native_support::{bytes, load_fixture_cases, proposal, vote};
use eve_consensus_comet::consensus::signing::{
    SigningError, encode_proposal_sign_bytes, encode_vote_sign_bytes,
};

#[test]
fn canonical_votes_match_independent_pinned_go_vectors() {
    let cases = load_fixture_cases("votes", "vote_cases");
    assert_eq!(cases.len(), 7);
    for case in cases {
        let mut candidate = vote(&case["vote"]);
        if !candidate.extension.is_empty() || !candidate.extension_signature.is_empty() {
            assert_eq!(
                encode_vote_sign_bytes(case["chain_id"].as_str().unwrap(), &candidate),
                Err(SigningError::UnsupportedVoteExtension)
            );
            candidate.extension.clear();
            candidate.extension_signature.clear();
        }
        assert_eq!(
            encode_vote_sign_bytes(case["chain_id"].as_str().unwrap(), &candidate).unwrap(),
            bytes(&case["sign_bytes"]),
            "{}",
            case["name"]
        );
    }
}

#[test]
fn canonical_proposals_match_independent_pinned_go_vectors() {
    let cases = load_fixture_cases("proposals", "proposal_cases");
    assert_eq!(cases.len(), 3);
    for case in cases {
        assert_eq!(
            encode_proposal_sign_bytes(
                case["chain_id"].as_str().unwrap(),
                &proposal(&case["proposal"])
            )
            .unwrap(),
            bytes(&case["sign_bytes"]),
            "{}",
            case["name"]
        );
    }
}

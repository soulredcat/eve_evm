// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native_support::{commit, header, load_fixture_cases, validators};
use eve_consensus_comet::{
    consensus::{
        authentication::ConsensusAuthenticationRequirement,
        certificates::{
            CertificateError, HistoricalValidatorSet, VerifiedClassicalCommit,
            verify_commit_certificate,
        },
    },
    wire::tendermint::types::{Commit, Header},
};

pub fn fixture(name: &str) -> (Header, Commit, HistoricalValidatorSet) {
    let case = load_fixture_cases("certificates", "certificates")
        .into_iter()
        .find(|case| case["name"] == name)
        .unwrap();
    let header = header(&case["header"]);
    let commit = commit(&case["commit"]);
    let set = HistoricalValidatorSet {
        height: header.height,
        authentication: ConsensusAuthenticationRequirement::ClassicalDev,
        validators: validators(&case["validators"]),
    };
    (header, commit, set)
}

pub fn verify(
    header: &Header,
    commit: &Commit,
    set: &HistoricalValidatorSet,
) -> Result<VerifiedClassicalCommit, CertificateError> {
    verify_commit_certificate(
        "eve-local-v1",
        3,
        2,
        commit.block_id.as_ref().unwrap(),
        header,
        commit,
        set,
    )
}

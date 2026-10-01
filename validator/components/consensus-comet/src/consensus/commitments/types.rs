// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Application execution height, distinct from a consensus-header height.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutionHeight(pub i64);

/// Heights affected by one `FinalizeBlock` result in the pinned engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommitmentHeightMapping {
    pub execution_height: ExecutionHeight,
    pub app_hash_header_height: i64,
    pub update_next_validators_hash_height: i64,
    pub updated_validator_set_height: i64,
    pub updated_last_commit_metadata_height: i64,
}

/// A successful hash/height comparison, without a certificate-authentication claim.
///
/// The caller must separately authenticate the header and historical validator set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeaderCommitmentMatch {
    pub execution_height: ExecutionHeight,
    pub consensus_header_height: i64,
}

/// Failures at the application-hash/height boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeightMappingError {
    NonPositiveExecutionHeight,
    HeightOverflow,
    WrongChain,
    WrongHeaderHeight,
    WrongApplicationHash,
}

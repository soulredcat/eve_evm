// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Distinct signing purposes for the experimental canonical envelope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AuthorizationPurpose {
    Enrollment = 1,
    ConsensusProposal = 2,
    ConsensusVote = 3,
    ConsensusCommit = 4,
    ValidatorTransition = 5,
    AccountOperation = 6,
    AccountRecovery = 7,
    Release = 8,
}

/// Expected enrollment selected by an authenticated registry, not supplied as untrusted proof data.
/// Construction alone does not authenticate enrollment, activation, or historical key transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnrolledHybridIdentity {
    pub identity: [u8; 32],
    pub key_epoch: u64,
    pub ed25519_public_key: [u8; 32],
    pub mldsa65_public_key: Vec<u8>,
}

/// Complete logical authorization data supplied by the calling protocol.
#[derive(Clone, Copy, Debug)]
pub struct HybridAuthorization<'a> {
    pub genesis_hash: [u8; 32],
    pub purpose: AuthorizationPurpose,
    pub payload: &'a [u8],
}

/// Wire signature components and the claimed identity; both signatures are always mandatory.
#[derive(Clone, Copy, Debug)]
pub struct HybridSignature<'a> {
    pub signer_identity: [u8; 32],
    pub key_epoch: u64,
    pub ed25519_signature: &'a [u8],
    pub mldsa65_signature: &'a [u8],
}

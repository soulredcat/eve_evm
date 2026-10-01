// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{Address, B256, Bytes, EvmStateRoot, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageProof {
    pub key: U256,
    pub value: U256,
    pub proof: Vec<Bytes>,
}

/// EVM membership only; no validator finality, system-root or Ethereum-chain claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountProof {
    pub height: u64,
    pub state_root: EvmStateRoot,
    pub address: Address,
    pub nonce: u64,
    pub balance: U256,
    pub code_hash: B256,
    pub storage_root: B256,
    pub account_proof: Vec<Bytes>,
    pub storage_proof: Vec<StorageProof>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProofLimits {
    pub maximum_requested_slots: usize,
    pub maximum_proof_bytes: usize,
    pub maximum_rebuild_bytes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateProofError {
    Limit(&'static str),
    Reservation { required: usize, reserved: usize },
    ArithmeticOverflow,
    RootMismatch,
    InvalidProof,
}

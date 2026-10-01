// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::B256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LaunchMode {
    Development,
    Production,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SecurityProfile {
    ClassicalDev = 1,
    HybridExperimental = 2,
    PqProfileVerified = 3,
}

/// A binding, not proof of authentication. Expected values must come from local
/// trusted genesis or verified applicable-height history, never the same peer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkProfileBinding {
    pub genesis_hash: B256,
    pub network_name: String,
    pub evm_chain_id: u64,
    pub protocol_version: u32,
    pub profile: SecurityProfile,
    pub activation_height: u64,
    pub key_epoch: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileError {
    InvalidBinding,
    WrongNetwork,
    WrongProtocol,
    WrongProfileHistory,
    UnsupportedHybrid,
}

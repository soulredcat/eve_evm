// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Conservative, versioned DEVELOPMENT metering; not a measured production price.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeGasSchedule {
    pub version: u32,
    pub dispatch: u64,
    pub calldata_zero_byte: u64,
    pub calldata_nonzero_byte: u64,
    pub proof_byte: u64,
    pub classical_verification: u64,
    pub mldsa65_verification: u64,
    pub storage_read: u64,
    pub storage_new_write: u64,
    pub storage_existing_write: u64,
    pub maximum_calldata_bytes: u64,
    pub maximum_proof_bytes: u64,
    pub maximum_signature_checks: u64,
    pub maximum_storage_operations: u64,
}

pub const NATIVE_GAS_V1: NativeGasSchedule = NativeGasSchedule {
    version: 1,
    dispatch: 5_000,
    calldata_zero_byte: 4,
    calldata_nonzero_byte: 16,
    proof_byte: 16,
    classical_verification: 3_000,
    mldsa65_verification: 100_000,
    storage_read: 2_100,
    storage_new_write: 20_000,
    storage_existing_write: 5_000,
    maximum_calldata_bytes: 131_072,
    maximum_proof_bytes: 65_536,
    maximum_signature_checks: 64,
    maximum_storage_operations: 256,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeGasInput {
    pub zero_calldata_bytes: u64,
    pub nonzero_calldata_bytes: u64,
    pub proof_bytes: u64,
    pub classical_verifications: u64,
    pub mldsa65_verifications: u64,
    pub storage_reads: u64,
    pub new_storage_writes: u64,
    pub existing_storage_writes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeMeterError {
    UnsupportedSchedule,
    LimitExceeded,
    ArithmeticOverflow,
}

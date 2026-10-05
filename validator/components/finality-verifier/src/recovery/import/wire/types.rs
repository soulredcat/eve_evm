// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{JournalDecodePreflight, StateBudget, StateError};

use crate::recovery::RecoveryError;

pub(super) const IMPORT_DOMAIN: &[u8] = b"EVE_IMPORT_V1";
/// Actual existing opaque recovery payload ceiling; fragmentation remains separate.
pub const MAXIMUM_IMPORT_WIRE_BYTES: usize = 4_198_312;

/// Unauthenticated immutable component views. Copies cannot change the sealed preflight.
#[derive(Clone, Copy, Debug)]
pub struct ImportWireSlices<'a> {
    pub journal: &'a [u8],
    pub execution: &'a [u8],
    pub finalized: &'a [u8],
    pub lookahead: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImportExecutionWireStats {
    pub encoded_bytes: usize,
    pub header_encoded_bytes: usize,
    pub transaction_count: usize,
    pub transaction_bytes: usize,
    pub receipt_count: usize,
    pub receipt_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImportNativeWireStats {
    pub encoded_bytes: usize,
    pub block_id_encoded_bytes: usize,
    pub header_encoded_bytes: usize,
    pub commit_encoded_bytes: usize,
    pub signature_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImportLookaheadWireStats {
    pub encoded_bytes: usize,
    pub frame: ImportNativeWireStats,
    pub transaction_count: usize,
    pub transaction_bytes: usize,
}

/// Exact logical wire/count admission facts, not allocation/RSS or authentication proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImportWireStats {
    pub encoded_bytes: usize,
    pub journal: JournalDecodePreflight,
    pub execution: ImportExecutionWireStats,
    pub finalized: ImportNativeWireStats,
    pub lookahead: ImportLookaheadWireStats,
}

/// Complete borrowed preflight bound to one immutable raw input and frozen budget.
/// Rust's borrow prevents caller mutation while this value is used. It creates no
/// state/root/certificate authority. Detached statistics cannot decode other bytes.
#[derive(Debug)]
pub struct ImportWirePreflight<'a> {
    pub(in crate::recovery::import::wire) bytes: &'a [u8],
    pub(in crate::recovery::import::wire) budget: StateBudget,
    pub(in crate::recovery::import::wire) slices: ImportWireSlices<'a>,
    pub(in crate::recovery::import::wire) stats: ImportWireStats,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImportWireError {
    State(StateError),
    Recovery(RecoveryError),
    BudgetExceeded,
    MalformedEncoding,
    NonCanonicalEncoding,
    AllocationFailed,
}

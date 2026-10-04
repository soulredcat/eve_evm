// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::CheckpointLimits;
use crate::recovery::import::wire::{
    ImportExecutionWireStats, ImportLookaheadWireStats, ImportWireError,
};
use eve_state::{StateBudget, StateError};

pub(super) const DOMAIN: &[u8] = b"EVE_CHECKPOINT_WITNESS_V1";
pub const MAXIMUM_CHECKPOINT_WITNESS_WIRE_BYTES: usize = 13 * 1_048_576;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointWitnessWireKind {
    Execution,
    Lookahead,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointWitnessWireStats {
    pub encoded_bytes: usize,
    pub version_encoded_bytes: usize,
    pub version_network_bytes: usize,
    pub native: ImportLookaheadWireStats,
    pub execution: Option<ImportExecutionWireStats>,
}

/// Exact immutable admission facts only; no finality, durability, freshness or lease authority.
#[derive(Debug)]
pub struct CheckpointWitnessWirePreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) budget: StateBudget,
    pub(super) limits: CheckpointLimits,
    pub(super) kind: CheckpointWitnessWireKind,
    pub(super) native: &'a [u8],
    pub(super) version: Option<&'a [u8]>,
    pub(super) execution: Option<&'a [u8]>,
    pub(super) stats: CheckpointWitnessWireStats,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckpointWitnessWireError {
    InvalidLimits,
    MalformedEncoding,
    NonCanonicalEncoding,
    BudgetExceeded,
    ReservationTooSmall,
    ArithmeticOverflow,
    AllocationFailed,
    State(StateError),
    ImportWire(ImportWireError),
    Recovery(crate::RecoveryError),
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256};
use eve_protocol_config::genesis::GenesisError;
use eve_protocol_config::records::RecordError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateError {
    Genesis(GenesisError),
    MissingHistory(u64),
    InvalidSchema,
    InvalidIdentity,
    MissingAccount(Address),
    MissingCode(B256),
    CodeHashMismatch(B256),
    NonCanonicalStorage,
    SystemKeyMismatch,
    SystemRecord(RecordError),
    ParentMismatch,
    HeightOverflow,
    VersionMismatch,
    CommitMismatch,
    BudgetExceeded,
    ArithmeticOverflow,
    MalformedEncoding,
    NonCanonicalEncoding,
}

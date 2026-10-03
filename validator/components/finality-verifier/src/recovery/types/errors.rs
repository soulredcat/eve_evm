// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::FinalityError;
use eve_consensus_comet::consensus::certificates::CertificateError;
use eve_evm::CompleteExecutionError;
use eve_state::StateError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecoveryError {
    State(StateError),
    Finality(FinalityError),
    Certificate(CertificateError),
    Execution(CompleteExecutionError),
    WrongParent,
    WrongIdentity,
    WrongHeight,
    WrongLookahead,
    UnsupportedValidatorTransition,
    UnknownProposer,
    InvalidExecutionTime,
    ReplayMismatch,
    MalformedEncoding,
    NonCanonicalEncoding,
    BudgetExceeded,
}

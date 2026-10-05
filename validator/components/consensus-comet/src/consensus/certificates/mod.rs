// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Classical native certificates under caller-authenticated applicable-height sets.
mod canonicalize_validator_set;
mod hash_consensus_header;
mod hash_transaction_data;
mod hash_validator_set;
mod hashing;
mod types;
mod validate_commit_context;
mod validate_consensus_header;
mod validate_validator_set;
mod validator_address;
mod verify_commit_certificate;
mod verify_commit_signature;
mod verify_native_ed25519_signature;

pub use canonicalize_validator_set::canonicalize_validator_set;
pub use hash_consensus_header::hash_consensus_header;
pub use hash_transaction_data::hash_transaction_data;
pub use hash_validator_set::hash_validator_set;
pub use types::{
    CertificateError, ClassicalValidator, HistoricalValidatorSet, MAX_DEVELOPMENT_VALIDATORS,
    VerifiedClassicalCommit,
};
pub use validator_address::validator_address;
pub use verify_commit_certificate::verify_commit_certificate;
pub use verify_native_ed25519_signature::verify_native_ed25519_signature;

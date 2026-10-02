// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Native classical history from explicit local trust; not EVE state or freshness proof.
mod initialize_genesis_history;
mod initialize_trusted_header_history;
mod types;
mod validate_header_successor;
mod verify_block_transaction_data;
mod verify_next_native_header;
mod views;

pub use initialize_genesis_history::initialize_genesis_history;
pub use initialize_trusted_header_history::initialize_trusted_header_history;
pub use types::{HistoryError, NativeHistoryVerifier, VerifiedNativeHeader};
pub use verify_next_native_header::verify_next_native_header;

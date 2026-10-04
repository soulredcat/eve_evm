// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Borrowed full-commit allocation admission; no state/root/finality authority.
mod decode_preflight_state_commit;
mod preflight_state_commit;
mod required_state_commit_decode_reservation;
mod scan_commit_accounts;
mod scan_commit_block;
mod scan_commit_byte_list;
mod scan_commit_codes;
mod scan_commit_history;
mod scan_commit_identity;
mod scan_commit_system;
mod state_commit_preflight_budget;
mod state_commit_preflight_bytes;
mod state_commit_preflight_stats;
mod types;
pub use decode_preflight_state_commit::decode_preflight_state_commit;
pub use preflight_state_commit::preflight_state_commit;
pub use required_state_commit_decode_reservation::required_state_commit_decode_reservation;
pub use state_commit_preflight_budget::state_commit_preflight_budget;
pub use state_commit_preflight_bytes::state_commit_preflight_bytes;
pub use state_commit_preflight_stats::state_commit_preflight_stats;
pub use types::{StateCommitDecodeStats, StateCommitPreflight};

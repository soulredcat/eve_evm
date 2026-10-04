// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod commit_preflight;
mod decode_state_journal;
mod journal_decoding;
mod preflight_state_version;
pub use decode_state_journal::decode_state_journal;
pub use journal_decoding::{JournalDecodePreflight, preflight_state_journal};
pub use preflight_state_version::preflight_state_version;

mod decode_accounts;
mod decode_block_payload;
mod decode_block_payload_from;
mod decode_codes;
mod decode_complete_state_data;
mod decode_history;
mod decode_identity;
mod decode_state_commit;
mod decode_state_version;
mod decode_system_map;
mod decode_system_record;
mod decode_system_value;
mod decode_value;
mod decode_version;
mod encode_block_payload;
mod encode_complete_state_data;
mod encode_identity;
mod encode_list;
mod encode_state_commit;
mod encode_state_data_with_history;
mod encode_state_version;
mod encode_version;
mod take_bytes;
mod take_list;

pub use decode_block_payload::decode_block_payload;
pub(crate) use decode_identity::decode_identity;
pub use decode_state_commit::decode_state_commit;
pub use decode_state_version::decode_state_version;
pub use decode_system_record::decode_system_record;
pub(crate) use decode_value::decode_value;
pub(crate) use take_bytes::take_bytes;
pub(crate) use take_list::take_list;

pub use encode_block_payload::encode_block_payload;
pub(crate) use encode_complete_state_data::encode_complete_state_data;
pub(crate) use encode_identity::encode_identity;
pub(crate) use encode_list::encode_list;
pub use encode_state_commit::encode_state_commit;
pub(crate) use encode_state_data_with_history::encode_state_data_with_history;
pub use encode_state_version::encode_state_version;
pub(crate) use encode_version::encode_version;

pub use commit_preflight::{
    StateCommitDecodeStats, StateCommitPreflight, decode_preflight_state_commit,
    preflight_state_commit, required_state_commit_decode_reservation,
    state_commit_preflight_budget, state_commit_preflight_bytes, state_commit_preflight_stats,
    state_commit_preflight_target_bytes,
};

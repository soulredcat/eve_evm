// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Native classical sign bytes; encoding grants no execution or signing approval.
mod canonical_block_id;
mod encode_proposal_sign_bytes;
mod encode_vote_sign_bytes;
mod normalize_timestamp;
mod types;
mod validate_chain_id;

pub(crate) use canonical_block_id::canonical_block_id;
pub use encode_proposal_sign_bytes::encode_proposal_sign_bytes;
pub use encode_vote_sign_bytes::encode_vote_sign_bytes;
pub(crate) use normalize_timestamp::normalize_timestamp;
pub use types::SigningError;
pub(crate) use validate_chain_id::validate_chain_id;

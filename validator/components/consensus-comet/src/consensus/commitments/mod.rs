// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Post-state and validator-update height mapping for CometBFT v0.40.0.

mod map_finalize_height;
mod match_next_header_commitment;
mod types;

pub use map_finalize_height::map_finalize_height;
pub use match_next_header_commitment::match_next_header_commitment;
pub use types::{
    CommitmentHeightMapping, ExecutionHeight, HeaderCommitmentMatch, HeightMappingError,
};

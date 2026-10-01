// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DevelopmentConsensusLimits;

/// One source for the values already frozen in the version-one genesis encoding.
pub fn development_consensus_limits() -> DevelopmentConsensusLimits {
    DevelopmentConsensusLimits {
        block_gas_limit: 30_000_000,
        maximum_block_bytes: 4_194_304,
        evidence_max_age_blocks: 1_000,
        evidence_max_age_seconds: 86_400,
    }
}

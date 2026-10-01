// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    application::{ConsensusApplication, application_finalization_binding},
    transport::proposals::VerifiedLocalEngineProposal,
};
use anyhow::{Result, ensure};

pub(super) fn validate_finalization_binding(
    application: &ConsensusApplication,
    source: &VerifiedLocalEngineProposal,
) -> Result<()> {
    let expected = application_finalization_binding(
        application,
        source.request().height,
        &source.request().proposer_address,
    )?;
    let provided = source.binding();
    ensure!(
        provided.genesis_hash == expected.genesis_hash
            && provided.chain_id == expected.chain_id
            && provided.authentication == expected.authentication
            && provided.key_epoch == expected.key_epoch
            && provided.proposer_owner == expected.proposer_owner
            && provided.previous_consensus_hash == expected.previous_consensus_hash,
        "finalized callback differs from retained application context"
    );
    Ok(())
}

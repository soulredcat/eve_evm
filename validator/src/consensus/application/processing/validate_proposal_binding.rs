// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    application::{ConsensusApplication, context::application_proposal_binding},
    transport::proposals::VerifiedLocalEngineProposal,
};
use anyhow::{Result, ensure};

pub(super) fn validate_proposal_binding(
    application: &ConsensusApplication,
    source: &VerifiedLocalEngineProposal,
) -> Result<()> {
    let expected = application_proposal_binding(application, &source.request().proposer_address)?;
    let provided = source.binding();
    ensure!(
        provided.genesis_hash == expected.genesis_hash
            && provided.chain_id == expected.chain_id
            && provided.authentication == expected.authentication
            && provided.key_epoch == expected.key_epoch
            && provided.proposer_owner == expected.proposer_owner
            && provided.previous_consensus_hash == expected.previous_consensus_hash,
        "local engine proposal differs from retained application context"
    );
    Ok(())
}

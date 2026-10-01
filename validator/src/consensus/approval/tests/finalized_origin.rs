// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::proposal_fixture;
use crate::consensus::{
    approval::{
        ApprovalError, approval_registry_status, create_approval_registry,
        create_execution_approval, publish_native_approval,
    },
    signing::{
        open_durable_signer,
        tests::{temporary_fixture, test_key},
    },
    transport::proposals::{EngineProposalBinding, fixture_verified_finalize_proposal},
};
use eve_state::development_state_budget;
use eve_storage::records::development_opaque_record_budget;
use std::sync::Arc;

#[test]
fn finalized_decision_can_execute_but_cannot_claim_unlocked_cache_replacement() {
    let (_directory, fixture) = temporary_fixture();
    let signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let template = proposal_fixture(&fixture.config, Vec::new());
    let binding = EngineProposalBinding {
        genesis_hash: fixture.config.genesis_hash,
        chain_id: fixture.config.chain_id.clone(),
        authentication: fixture.config.authentication,
        key_epoch: fixture.config.key_epoch,
        proposer_owner: template.binding().proposer_owner,
        previous_consensus_hash: [0; 32],
    };
    let source = fixture_verified_finalize_proposal(template.request().clone(), binding);
    let approval = Arc::new(
        create_execution_approval(
            &signer,
            &source,
            &development_state_budget(),
            256 * 1024 * 1024,
        )
        .unwrap(),
    );
    let registry = create_approval_registry();
    assert!(matches!(
        publish_native_approval(&registry, &signer, source, approval),
        Err(ApprovalError::Unavailable { .. })
    ));
    assert_eq!(approval_registry_status(&registry).unwrap().full, 0);
    assert_eq!(approval_registry_status(&registry).unwrap().raw, 0);
}

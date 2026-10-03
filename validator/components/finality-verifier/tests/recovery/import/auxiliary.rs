// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use alloy_primitives::keccak256;
use eve_finality_verifier::{
    ImportError, imported_state_anchor, imported_state_commit, imported_transition_state,
    initialize_authenticated_import, into_imported_state, prepare_authenticated_import,
};
use eve_state::{B256, Bytes, JournalOperation, SecurityProfile, development_state_budget};

use super::support::input;
use crate::recovery_support::{self as support, CLONE_BYTES};

#[test]
fn unused_hash_checked_code_changes_only_local_content_identity_and_requires_exact_next_parent() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    let mut delta = input(&chain, 1);
    let code = Bytes::from_static(&[0x60, 0x01, 0x00]);
    let code_hash = keccak256(&code);
    delta
        .journal
        .operations
        .insert(0, JournalOperation::PutCode { code_hash, code });
    let transition =
        prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES).unwrap();
    let imported = imported_transition_state(&transition);
    let actual = imported_state_commit(imported);
    let canonical = &chain.commits[1].target;
    assert_eq!(actual.target.evm_root, canonical.evm_root);
    assert_eq!(actual.target.system_root, canonical.system_root);
    assert_eq!(actual.target.execution_hash, canonical.execution_hash);
    assert_eq!(actual.target.application, canonical.application);
    assert_ne!(actual.target.content_digest, canonical.content_digest);
    assert!(actual.state.codes.contains_key(&code_hash));
    assert!(imported_state_anchor(imported).is_some());
    let imported = into_imported_state(transition);
    assert_eq!(
        prepare_authenticated_import(&imported, Arc::new(input(&chain, 2)), &budget, CLONE_BYTES)
            .unwrap_err(),
        ImportError::WrongParent,
    );
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn exact_local_parent_refuses_digest_time_identity_profile_epoch_or_height_substitution() {
    let chain = support::recovery_chain();
    let budget = development_state_budget();
    let parent = initialize_authenticated_import(&chain.genesis, &budget).unwrap();
    for field in 0..10 {
        let mut delta = input(&chain, 1);
        let version = &mut delta.journal.parent;
        match field {
            0 => version.content_digest = B256::repeat_byte(0x11),
            1 => version.timestamp += 1,
            2 => version.identity.network_name.push_str("-foreign"),
            3 => version.identity.evm_chain_id += 1,
            4 => version.identity.genesis.0 = B256::repeat_byte(0x12),
            5 => version.identity.configuration_digest = B256::repeat_byte(0x13),
            6 => version.identity.protocol_version += 1,
            7 => version.identity.security_profile = SecurityProfile::HybridExperimental,
            8 => version.identity.key_epoch += 1,
            9 => version.height = u64::MAX,
            _ => unreachable!(),
        }
        assert_eq!(
            prepare_authenticated_import(&parent, Arc::new(delta), &budget, CLONE_BYTES)
                .unwrap_err(),
            ImportError::WrongParent,
        );
    }
    assert_eq!(imported_state_commit(&parent), &chain.commits[0]);
}

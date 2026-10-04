// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::Fixture;
use eve_development_fixtures::recovery::CLONE_BYTES;
use eve_finality_verifier::{
    decode_logical_import_wire, imported_state_commit, initialize_authenticated_import,
    into_imported_state, preflight_logical_import_wire, prepare_authenticated_import,
};
use eve_state::development_state_budget;
use eve_storage::records::{OpaqueRecordRepository, read_opaque_record};
use std::sync::Arc;

pub fn authenticate_retained(repository: &OpaqueRecordRepository, fixture: &Fixture) {
    let budget = development_state_budget();
    let mut current =
        Arc::new(initialize_authenticated_import(&fixture.chain.genesis, &budget).unwrap());
    for height in 1..=2 {
        let retained = read_opaque_record(repository, height as u64)
            .unwrap()
            .unwrap();
        assert_eq!(retained.payload, fixture.wires[height - 1]);
        let framing = preflight_logical_import_wire(&retained.payload, &budget).unwrap();
        let input = decode_logical_import_wire(&framing).unwrap();
        let transition =
            prepare_authenticated_import(&current, Arc::new(input), &budget, CLONE_BYTES).unwrap();
        current = into_imported_state(transition);
        // Complete oracle comparison includes both roots, receipts, nonce/balance,
        // deterministic fee/system effects and canonical auxiliary representation.
        assert_eq!(
            imported_state_commit(&current),
            &fixture.chain.commits[height]
        );
    }
    assert!(!fixture.chain.commits[1].block.transactions.is_empty());
    assert!(!fixture.chain.commits[1].block.receipts.is_empty());
}

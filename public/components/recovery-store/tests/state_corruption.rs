// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod state_support;
use eve_state::Address;
use eve_storage::state::{commit_state, development_state_storage_budget, open_state_repository};

#[test]
fn missing_code_account_slot_system_payload_root_or_marker_rejects_reopen() {
    for kind in [
        "code",
        "account",
        "slot",
        "system",
        "commit",
        "header",
        "transactions",
        "receipts",
        "roots",
        "commit-id",
        "marker",
    ] {
        let directory = tempfile::tempdir().unwrap();
        let genesis = state_support::genesis();
        let first = state_support::next(&genesis, 1);
        let budget = development_state_storage_budget();
        let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
        commit_state(&mut store, &first).unwrap();
        drop(store);
        let key = match kind {
            "code" => [
                b"eve/state/v1/code/".as_slice(),
                first.state.accounts[&Address::repeat_byte(5)]
                    .code_hash
                    .as_slice(),
            ]
            .concat(),
            "account" => [
                b"eve/state/v1/account/".as_slice(),
                Address::repeat_byte(5).as_slice(),
            ]
            .concat(),
            "slot" => [
                b"eve/state/v1/slot/".as_slice(),
                Address::repeat_byte(5).as_slice(),
                &[0; 32],
            ]
            .concat(),
            "system" => [
                b"eve/state/v1/system/".as_slice(),
                first.state.system.keys().next().unwrap().as_slice(),
            ]
            .concat(),
            "marker" => b"eve/state/v1/durable-head".to_vec(),
            _ => [
                format!("eve/state/v1/{kind}/").as_bytes(),
                &1u64.to_be_bytes(),
            ]
            .concat(),
        };
        let raw = rocksdb::DB::open_default(directory.path()).unwrap();
        raw.delete(key).unwrap();
        raw.flush_wal(true).unwrap();
        drop(raw);
        assert!(
            open_state_repository(directory.path(), &genesis, budget).is_err(),
            "missing {kind}"
        );
    }
}

#[test]
fn wrong_genesis_configuration_or_schema_is_never_silently_initialized() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let budget = development_state_storage_budget();
    drop(open_state_repository(directory.path(), &genesis, budget).unwrap());
    let mut other = genesis.clone();
    other.target.identity.configuration_digest[0] ^= 1;
    assert!(open_state_repository(directory.path(), &other, budget).is_err());
    let raw = rocksdb::DB::open_default(directory.path()).unwrap();
    raw.put(b"eve/state/v1/schema", b"unsupported schema")
        .unwrap();
    raw.flush_wal(true).unwrap();
    drop(raw);
    assert!(open_state_repository(directory.path(), &genesis, budget).is_err());
}

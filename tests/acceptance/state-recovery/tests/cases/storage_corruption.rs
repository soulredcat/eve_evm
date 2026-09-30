use crate::support;

use alloy_primitives::{Address, U256};
use eve_storage::state::{commit_state, open_state_repository};
use rocksdb::{DB, WriteOptions};
use support::{
    execution::{executed_commit, execution_fixture},
    storage::{budget, database_path, height_key, key, local_directory},
};

#[test]
fn ts05_missing_or_corrupt_account_and_slot_rows_never_become_zero_balances() {
    let (genesis, next) = executed_commit();
    let contract: Address = execution_fixture()["contract"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    for kind in ["account", "slot"] {
        let directory = local_directory("row-corruption-");
        let path = database_path(&directory);
        let mut repository = open_state_repository(&path, &genesis, budget()).unwrap();
        commit_state(&mut repository, &next).unwrap();
        drop(repository);
        let raw = DB::open_default(&path).unwrap();
        let mut sync = WriteOptions::default();
        sync.set_sync(true);
        if kind == "account" {
            raw.delete_opt(key(b"eve/state/v1/account/", contract.as_slice()), &sync)
                .unwrap();
        } else {
            let slot_key = key(
                b"eve/state/v1/slot/",
                &[contract.as_slice(), &U256::ZERO.to_be_bytes::<32>()].concat(),
            );
            raw.put_opt(slot_key, [0_u8; 32], &sync).unwrap();
        }
        drop(raw);
        assert!(
            open_state_repository(&path, &genesis, budget()).is_err(),
            "{kind}"
        );
    }
}

#[test]
fn ts05_code_blob_and_receipt_reference_corruption_rejects_the_namespace() {
    let (genesis, next) = executed_commit();
    let contract: Address = execution_fixture()["contract"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let hash = next.state.accounts[&contract].code_hash;
    for kind in ["code", "receipt"] {
        let directory = local_directory("blob-corruption-");
        let path = database_path(&directory);
        let mut repository = open_state_repository(&path, &genesis, budget()).unwrap();
        commit_state(&mut repository, &next).unwrap();
        drop(repository);
        let raw = DB::open_default(&path).unwrap();
        let mut sync = WriteOptions::default();
        sync.set_sync(true);
        if kind == "code" {
            raw.put_opt(key(b"eve/state/v1/code/", hash.as_slice()), [0_u8], &sync)
                .unwrap();
        } else {
            raw.delete_opt(height_key(b"eve/state/v1/receipts/", 1), &sync)
                .unwrap();
        }
        drop(raw);
        assert!(
            open_state_repository(&path, &genesis, budget()).is_err(),
            "{kind}"
        );
    }
}

#[test]
fn ts05_missing_retained_history_is_an_error_not_a_pruning_claim() {
    let (genesis, next) = executed_commit();
    for prefix in [
        b"eve/state/v1/commit/".as_slice(),
        b"eve/state/v1/header/".as_slice(),
        b"eve/state/v1/execution-hash/".as_slice(),
    ] {
        let directory = local_directory("history-corruption-");
        let path = database_path(&directory);
        let mut repository = open_state_repository(&path, &genesis, budget()).unwrap();
        commit_state(&mut repository, &next).unwrap();
        drop(repository);
        let raw = DB::open_default(&path).unwrap();
        let mut sync = WriteOptions::default();
        sync.set_sync(true);
        raw.delete_opt(height_key(prefix, 0), &sync).unwrap();
        drop(raw);
        assert!(open_state_repository(&path, &genesis, budget()).is_err());
    }
}

#[test]
fn ts05_durable_marker_identity_cannot_be_detached_from_its_whole_commit() {
    let (genesis, next) = executed_commit();
    let directory = local_directory("marker-corruption-");
    let path = database_path(&directory);
    let mut repository = open_state_repository(&path, &genesis, budget()).unwrap();
    commit_state(&mut repository, &next).unwrap();
    drop(repository);
    let raw = DB::open_default(&path).unwrap();
    let mut marker = raw.get(b"eve/state/v1/durable-head").unwrap().unwrap();
    *marker.last_mut().unwrap() ^= 1;
    let mut sync = WriteOptions::default();
    sync.set_sync(true);
    raw.put_opt(b"eve/state/v1/durable-head", marker, &sync)
        .unwrap();
    drop(raw);
    assert!(open_state_repository(&path, &genesis, budget()).is_err());
}

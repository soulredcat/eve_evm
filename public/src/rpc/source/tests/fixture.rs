// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::mempool::{MempoolLimits, start_applied_mempool};
use crate::rpc::{AppliedRpcConfig, RpcContext, create_applied_rpc_context};
use crate::sync::applied::{
    AppliedMode, AppliedOwner, AppliedReader, capture_applied_state,
    open_applied_state_service_with_mode,
    tests::import_fixtures::{ImportChain, import_chain, import_config, signed_transaction},
};
use eve_state::{Address, StateBudget};
use std::{path::Path, sync::Arc};

pub(super) struct AppliedRpcFixture {
    pub owner: AppliedOwner,
    pub reader: AppliedReader,
    pub context: Arc<RpcContext>,
    pub chain: ImportChain,
    pub budget: StateBudget,
    pub sender: Address,
    pub contract: Address,
    pub _directory: tempfile::TempDir,
}

/// Reuse the actual canonical/native import fixture; no new proof generator or fake state service.
pub(super) fn fixture(retained_records: Option<u64>) -> AppliedRpcFixture {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let local = root.join("local-tests/b4-rpc-source");
    std::fs::create_dir_all(&local).unwrap();
    let directory = tempfile::tempdir_in(local).unwrap();
    let chain = import_chain();
    let mut config = import_config(&directory.path().join("applied"), &chain);
    if let Some(limit) = retained_records {
        config.repository_budget.maximum_retained_records = limit;
    }
    let budget = config.state_budget;
    let (owner, reader) = open_applied_state_service_with_mode(
        config,
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let pool = start_applied_mempool(
        capture_applied_state(&reader).unwrap(),
        MempoolLimits::default(),
    );
    let context = create_applied_rpc_context(
        reader.clone(),
        pool,
        AppliedRpcConfig {
            state_budget: budget,
            zone: eve_node_policy::ZoneId(1),
            buffer_bytes: 32 * 1_048_576,
            maximum_simulation_memory_bytes: 8 * 1_048_576,
        },
    )
    .unwrap();
    let sender = eve_evm::decode_signed_transaction(&signed_transaction(), 31_337, 131_072)
        .unwrap()
        .sender();
    AppliedRpcFixture {
        owner,
        reader,
        context,
        chain,
        budget,
        sender,
        contract: Address::with_last_byte(0x42),
        _directory: directory,
    }
}

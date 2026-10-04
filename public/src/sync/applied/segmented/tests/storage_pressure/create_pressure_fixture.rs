// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    assert_storage_saturation::assert_storage_saturation,
    pressure_configuration::pressure_configuration,
    start_measurement_http::start_measurement_http,
    types::{MeasurementContract, PressureFixture},
};
use crate::{
    mempool::{MempoolLimits, start_applied_mempool},
    rpc::{AppliedRpcConfig, create_applied_rpc_context},
    sync::applied::{
        capture_applied_state, open_segmented_applied_state_service,
        segmented::tests::fixtures::logical_chain, tests::import_fixtures::signed_transaction,
    },
};
use std::sync::Arc;

pub(super) fn create_pressure_fixture(
    runtime: &tokio::runtime::Runtime,
    contract: &MeasurementContract,
) -> PressureFixture {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("store");
    let chain = logical_chain(false);
    let config = pressure_configuration(&path, &chain, contract);
    assert_eq!(
        config.application.public_budget.maximum_working_state_bytes,
        contract.working_estimate_limit_bytes
    );
    assert_eq!(
        config.application.public_budget.worker_cpu_basis_points,
        contract.worker_cpu_basis_points
    );
    let (owner, reader) = open_segmented_applied_state_service(config, &chain.genesis).unwrap();
    let genesis = capture_applied_state(&reader).unwrap();
    assert_storage_saturation(&owner, &reader, &chain);
    let pool = start_applied_mempool(Arc::clone(&genesis), MempoolLimits::default());
    let context = create_applied_rpc_context(
        reader.clone(),
        pool,
        AppliedRpcConfig {
            state_budget: eve_state::development_state_budget(),
            zone: eve_node_policy::ZoneId(1),
            buffer_bytes: 32 * 1_048_576,
            maximum_simulation_memory_bytes: contract.simulation_arena_bytes,
        },
    )
    .unwrap();
    let (clients, servers) =
        runtime.block_on(start_measurement_http(Arc::clone(&context), contract));
    let sender = eve_evm::decode_signed_transaction(&signed_transaction(), 31_337, 131_072)
        .unwrap()
        .sender();
    PressureFixture {
        directory,
        path,
        chain,
        owner,
        reader,
        genesis,
        context,
        clients,
        servers,
        sender,
    }
}

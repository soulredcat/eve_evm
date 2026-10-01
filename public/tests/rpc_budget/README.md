<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Independent public RPC resource tests

Canonical owner: B2 correctness/security tests, wired by the public runtime owner
under `cfg(test)`. These cases inspect private RPC resource contracts without
exporting an admin interface or raw production database handle.

Fixtures create actual validated disposable genesis and persistent repositories
under ignored root `local-tests/b2-rpc-budget/`. Random development keys stay in
memory; persisted genesis and block state contain public data. Tests use the real
RPC dispatcher, execution engine, mempool and recovery repository.

The worker cases acquire the exact configured limits and assert explicit rejection
and release, including the producer's reserved capacity. The cancellation case
starts a real gas-bounded EVM loop, cancels its async caller, and asserts execution
and byte leases remain held until the blocking worker exits. Ordinary local reads
must remain available. It is not a test of BFT consensus starvation or a measured
RSS guarantee, and it does not use an injected sleeping worker as EVM evidence.

Proof/storage requests test standard zero-padded 32-byte words while preserving
strict block quantities. Proof slot count, simulation gas/calldata and unavailable
history return genuine errors. The exact 1000/1001-block log boundary uses 1001
real synced local empty blocks rather than changing a mock head height.

Run the registered B2 gate, or for an individual diagnostic run
`cargo test --locked -p eve-public --lib rpc::tests:: -- --test-threads=1`.
Filtered or individual results are not a full bulk pass. No validator finality,
production signing, bridge custody, hardware power loss or TPS claim follows
from these local tests.

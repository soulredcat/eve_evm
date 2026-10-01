<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Master runtime

The B1 storage and B2 RPC development composition are implemented; the production master follower,
network replication and authenticated source recovery remain later work.

## Development composition

`serve-dev` composes the canonical public library with an owned Tokio runtime;
master does not copy private public startup, EVM or commitment behavior:

```sh
cargo run --locked -p eve-master -- serve-dev --mode DEV_ALL_IN_ONE --acknowledge-unsafe-development --root . --data local-tests/master-rpc --genesis local-tests/development.json --http-address 127.0.0.1:8545 --ws-address 127.0.0.1:8546 --block-interval-ms 1000 --zone-id 1
```

Startup emits actual bound addresses and LOCAL_DEV_UNAUTHENTICATED status.
Ctrl-C stops the owned producer and RPC listeners; restart reconciles the same
durable namespace before serving. Production/sync-only/missing-acknowledgement
paths reject. The actual developer fixture tests signed submission/receipt,
retained history and restart through this composition and two public processes.
It establishes no validator finality. RPC, logical memory and separate transport
limits follow [public's profile](../public/README.md); default external binding
is disabled. Zone metadata grants no voting or shard ownership. B6 standalone
packaging and B3/B4 production roles remain separate unimplemented gates.

The eve-master binary owns only local development orchestration. It reuses the
canonical validator state/execution/protocol components and public recovery
repository. Init, inspect, real EVM block application, snapshot export and restore
all require explicit DEV_ALL_IN_ONE and unsafe-development acknowledgement. A
production/master-sync-only mode cannot activate the local producer. No private
key is supplied or embedded by this CLI; its genesis input accepts public keys,
funded fake balances and bounded code. Unknown/private-signing fields fail.

Data and snapshot paths must be dedicated ignored local-tests descendants;
existing namespaces are reconciled, never overwritten by initialization/import.
The configured complete-state baseline is 8 MiB with a 16 MiB clone reservation,
16 MiB commit limit, bounded RocksDB allocations and limited snapshot leases.
These bounds are development policy, not a mainnet capacity/RSS claim.

```sh
cargo run --locked -p eve-master -- init-dev --mode DEV_ALL_IN_ONE --acknowledge-unsafe-development --root . --data local-tests/master-dev --genesis local-tests/development.json
cargo run --locked -p eve-master -- inspect-dev --mode DEV_ALL_IN_ONE --acknowledge-unsafe-development --root . --data local-tests/master-dev --genesis local-tests/development.json
cargo run --locked -p eve-master -- apply-dev --mode DEV_ALL_IN_ONE --acknowledge-unsafe-development --root . --data local-tests/master-dev --genesis local-tests/development.json --block local-tests/block.json
cargo run --locked -p eve-master -- snapshot-dev --mode DEV_ALL_IN_ONE --acknowledge-unsafe-development --root . --data local-tests/master-dev --genesis local-tests/development.json --output local-tests/master-snapshot
cargo run --locked -p eve-master -- restore-dev --mode DEV_ALL_IN_ONE --acknowledge-unsafe-development --root . --data local-tests/master-restored --genesis local-tests/development.json --source local-tests/master-snapshot
```

The genesis JSON requires schema/protocol versions, eve-local-v1/31337 identity,
initial agreed timestamp, CLASSICAL_DEV profile, accounts with address/funded
balance/nonce/code, and four validators with owner/full lowercase public-key hex,
self-bond and power. Economics use the frozen development contract; supply and
escrow validation are canonical. Block JSON carries agreed timestamp and ordered
signed transaction hex envelopes. Runtime keys/faucet tooling belongs to the
development network fixture, not a built-in production configuration.

Status explicitly reports LOCAL_DURABLE_ONLY_NO_VALIDATOR_FINALITY. Successful
execution/storage/root matching does not create a consensus certificate. The
full B1 gate verifies its root/replay/recovery/guard integration; complete public,
validator and master production entry points and role distributions are later.

Production role: verify finalized consensus provenance, import authenticated state transitions, maintain durable state/history, publish snapshots, support recovery and report replication lag. The master has no production voting/proposal authority and is not a mandatory transaction hop.

A local development all-in-one harness may compose shared execution and test-validator components. It must not leak that authority into MASTER_SYNC_ONLY.

The master host/distribution may explicitly run master, public and validator components together. Keep their entry points, configuration, credentials and authority separate; composing a validator on the same host does not make the master role a voter. Public and validator distributions must each be independently copyable/buildable/runnable without this private master implementation. Reproducible packaging includes only the required canonical reusable components and never creates a second hand-maintained consensus implementation.

Start with one master and one public runtime alongside the four-validator devnet baseline. One master can serve several public nodes through bounded object distribution or relays. A single-validator harness is local development only.

Introduce `zone_id` as routing/operational metadata from the start, distinct from network name, immutable genesis hash and EVM chain ID. Zones do not grant voting power, state ownership or finality. Evolve toward two mutually synchronized independent finalized-history replicas, then a planned ten masters across regions; this is not deployment, hardware purchase or launch authorization.

Every master verifies validator finality, historical set transitions and commitment binding independently, including imports from another master. Use separate local storage namespaces; mutual sync never merges writable EVM states. Fence writers that share one mutable namespace without requiring a global writable-database lease across independent replicas. Report actual finalized, applied, durable and authenticated-snapshot heights.

Expose logical sync endpoints/relays without public database/admin access or privileged master inventory. Public nodes select eligible sources by authenticated identity/network/profile/proofs and verified lag/data availability before measured service latency and verified-data throughput. Support bounded fallback and resume; no endpoint-anonymity guarantee or mandatory per-transaction master acknowledgement exists.

Implement against shared `FinalityVerifier`, `StateStore`, `BlockStore` and sync interfaces. Read plans [02](../docs/plan/02-state-and-storage.md), [10](../docs/plan/10-master-public-node-model.md), [14](../docs/plan/14-block-and-state-commitment-spec.md), [15](../docs/plan/15-network-and-sync-protocol.md), [16](../docs/plan/16-genesis-upgrade-and-recovery.md), [19](../docs/plan/19-security-and-release-engineering.md), [22](../docs/plan/22-code-layout-and-dependency-policy.md) and [32](../docs/plan/32-regional-masters-and-public-persistence.md) before implementation.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Runtime architecture and authority

These are implementation obligations. Consult [execution status](../execution/README.md)
for achieved coverage; this contract does not certify complete runtime integration.

## Canonical ownership

| Directory | Owns | Required dependency boundary |
|---|---|---|
| `public/` | Public RPC/P2P, admission, verified RAM views, readiness, bootstrap and persistence orchestration | No private master implementation, including transitive dependencies |
| `master/` | Private follower/archive, replication and protected snapshot/storage orchestration | Imports verified finalized history; never decides finality |
| `validator/` | Execution validation, proposals, consensus voting/signing and validator lifecycle | No private master implementation, including transitive dependencies |

Public owns `components/recovery-store` and `components/node-policy`. Validator
owns `components/state`, `execution`, `authentication`, `consensus-comet` and
`protocol-config`. Each component has a responsibility README. Runtime entry
points remain thin; do not duplicate consensus-critical behavior or introduce
root `crates/`, `create/` or generic dumping directories.

Validators execute and validate before voting. Necessary data and durable
anti-double-sign state must exist before releasing a signature. Master storage,
release signatures and public-node count never grant voting or finality power.
Loss of quorum stalls new finality; it never permits master takeover or automatic
quorum reduction. The classical baseline requires strictly more than two-thirds
valid weighted power and assumes less than one-third Byzantine weighted power.

## Public RAM and durable recovery

Default public nodes keep active verified state in RAM and finalized recovery
blocks/checkpoints durable through an isolated bounded storage worker. Separate
execution-applied, durable-recovery and authenticated-state heights. A queue or
OS cache write is not durability, and a matching received root alone is not proof
of correct execution without authenticated commitments or replay.

Storage workers must not hold RAM state locks during disk or network waits.
Bound queues, memory, lag, snapshot work and historical reads. Measure resource
interference; thread separation does not prove zero overhead. Preserve the last
recoverable finalized copy. Failed persistence or unavailable replay history must
produce explicit degraded/unavailable readiness instead of invented state.

Ordinary transactions and finality do not wait for master acknowledgements.
Public recovery must work while masters are offline when authenticated durable
peer data, retention capacity and the required validator quorum remain available.
Validator signing durability remains mandatory regardless of RAM optimizations.

## Regional sources and master replication

The intended separated development topology starts with one master, one public
node and the four-validator baseline. Later stages verify two independent master
stores and a planned ten-master regional layout. These intentions authorize no
production deployment and demonstrate no throughput multiplier.

Public nodes choose a preferred eligible sync endpoint and retain fallback peers;
they do not need private master database access or inventory. Verify network,
identity, applicable profile and history before ranking latency, load or lag.
Ping speed is not authority. `zone_id` controls routing/placement, never chain
identity, shard ownership or voting power. Master replicas independently verify
imported finalized history; competing database writes never choose canonical state.

## Distribution, execution and scope

A master host may compose all three runtimes with separate credentials and
authority. `MASTER_SYNC_ONLY` never votes. Public and validator distributions
must eventually build and run after copying only their role directory to an
unrelated clean location. Include dependencies, manifests/locks, pins, sanitized
examples and notices without escaping path dependencies. Package canonical
sources reproducibly; do not maintain independent duplicated implementations.
This standalone gate remains `NOT_IMPLEMENTED` / `NOT_RUN`.

Preserve deterministic EVM semantics, atomicity, ordering and the 40% burn / 30%
node / 30% validator fee policy. Consensus arithmetic uses integers; execution
uses no network calls, wall-clock reward scoring or random map iteration.
The 1M aggregate finalized TPS goal requires sustained measured evidence with
real persistence and the accepted security profile. Adding replicas is not proof.

EVE core has no external-chain execution, voting, finality or durable-acknowledgement
dependency. D40 defers bridges and adapters until EVE testnet and later owner scope;
separate programs adapt to EVE. Local EVM/RPC reference fixtures remain core.
Classical development authentication is not PQ-secure. Activated hybrid profiles
must enforce both signatures for the same enrolled identity and message; a wrapper
or post-finality stamp does not satisfy the core security requirements.

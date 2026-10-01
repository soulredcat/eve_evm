<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 02 — State and storage

## Storage is an implementation detail; commitments are protocol rules

The default `PUBLIC` profile keeps hot execution/query state in RAM and persists finalized blocks plus recoverable state checkpoints locally. RAM-first operation is not RAM-only retention: working views, caches and overlays have explicit capacity budgets. A RAM-only non-voting replica is an explicit development/ephemeral alternative, not the default. See [plan 32](32-regional-masters-and-public-persistence.md); this is a specification, not implemented persistence.

The development baseline is RocksDB behind `StateStore` and `BlockStore` interfaces; benchmark MDBX later using identical workloads before replacing it. No engine is declared smallest or fastest without measurements. Master followers use durable NVMe-backed storage under a measured sync/recovery policy, outside the mandatory transaction path. NVMe generation is an operator hardware preference, not a protocol requirement.

Preserve EVM account balances, nonces, bytecode, arbitrary contract storage and system accounting. An AMM-specific fixed struct may be an index/cache, never a replacement for arbitrary Solidity storage semantics.

## Durability contract

Separate atomic application of a verified finalized height in RAM from its durable storage commit. At each full `StateStore` durable commit, atomically persist state changes, block/receipt references, protocol metadata and the durable height marker under the required database WAL/sync policy. An append-only block log may serve a distinct replay/availability purpose; it does not replace atomic reference/height publication.

An isolated asynchronous public storage worker consumes ordered immutable finalized batches. Reserve bounded handoff capacity before advancing RAM application. One durable commit owner controls each local database namespace. Execution/query state locks are never held across disk writes or fsync. Publish a durable acknowledgment only after the required storage sync succeeds, not when a batch is enqueued. Public RPC may expose a locally verified RAM-applied view within the explicit readiness/lag policy while reporting finalized, applied, durable and authenticated state heights separately; finality does not automatically authenticate that height's post-state under plan 12's H/H+1 binding.

An explicit public recovery-store profile may persist finalized block payloads, authentication, protocol/configuration, security-profile and validator-set history, and consistent periodic checkpoints instead of writing the full hot state every height. Its `durable_height` advances only after payloads, references and recovery metadata are synced and a complete verified checkpoint-to-height replay sequence exists. Report `checkpoint_height` and full `StateStore` durable height separately when applicable; this recovery watermark does not relax atomic full-state commits. Restart must replay and check the resulting commitments.

Bound storage queues by bytes, batch count and oldest-item age. Budget worker CPU, database/page cache, immutable buffer retention, snapshots and compaction. B0 freezes versioned measured configuration and saturation scenarios; no arbitrary throughput or zero-overhead guarantee is implied. Apply backpressure before lag threatens configured limits; stop readiness or local ingestion and recover from durable peers when necessary rather than growing RAM without bound.

Do not expose a state as durably stored because it reached RAM or the OS page cache. Test process termination and simulated power-loss/torn-write conditions. Recover any lost queued tail from authenticated durable peers after the last complete local commit; replay finalized effects exactly once. Never prune the only recoverable finalized data to keep a queue or throughput graph small.

## Role-specific retention

- Master: durable current state, authenticated history, snapshots, archive policy and checksummed recovery records.
- Validator: durable consensus WAL, anti-double-sign records, current state or recoverable checkpoints, and recent finalized block data independent of masters. Signing safety must be persisted before releasing a signature; asynchronous public persistence grants no exception.
- Non-voting public replica: RAM-first working state with durable finalized blocks/checkpoints by default; an explicitly ephemeral replica must fully reverify after loss and cannot provide validator signing safety.

## Space controls

Use immutable history segments, measured compression and explicit indexes. Prune by finalized retention watermarks only after the recovery policy is satisfied. Snapshots are consistent views at one verified height, published atomically with manifests. Keep resource budgets for compaction, cache, RPC history and concurrent snapshots.

Track logical state bytes, physical database bytes, history bytes, write amplification, storage queue bytes/count/age, apply-to-durable lag, snapshot bandwidth, compaction stalls and recovery time separately. Storage workers still consume shared CPU, memory bandwidth and I/O; measure execution/RPC latency under concurrent persistence. A smaller file after unsafe pruning is not an optimization.

Detailed encodings/commit protocol: [14](14-block-and-state-commitment-spec.md). Availability/retention: [15](15-network-and-sync-protocol.md). Crash tests: [20](20-test-vectors-and-acceptance.md).

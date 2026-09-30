# 02 — State and storage

## Storage is an implementation detail; commitments are protocol rules

Use a bounded RAM cache/working overlay over durable state. The development baseline is RocksDB behind `StateStore` and `BlockStore` interfaces; benchmark MDBX later using identical workloads before replacing it. No engine is declared smallest or fastest without measurements. NVMe Gen5 is an operator hardware preference, not a protocol requirement.

Preserve EVM account balances, nonces, bytecode, arbitrary contract storage and system accounting. An AMM-specific fixed struct may be an index/cache, never a replacement for arbitrary Solidity storage semantics.

## Durability contract

At each application commit, atomically persist state changes, block/receipt references, protocol metadata and a durable height marker. Use the database's WAL with the required sync policy; do not add a second custom WAL without a documented ordering/recovery need. An append-only block log can serve a different replay/availability purpose.

Do not expose a state as durably stored because it reached RAM or the OS page cache. Test process termination and simulated power-loss/torn-write conditions. If a finalized consensus block was received before application persistence completed, recovery must replay it exactly once, not invent a new canonical result.

## Role-specific retention

- Master: durable current state, authenticated history, snapshots, archive policy and checksummed recovery records.
- Validator: durable consensus WAL, anti-double-sign records, current state or recoverable checkpoints, and recent finalized block data independent of masters.
- Non-voting public replica: RAM-heavy or disk-backed mode; an ephemeral replica must fully reverify after loss and cannot provide validator signing safety.

## Space controls

Use immutable history segments, measured compression and explicit indexes. Prune by finalized retention watermarks only after the recovery policy is satisfied. Snapshots are consistent views at one verified height, published atomically with manifests. Keep resource budgets for compaction, cache, RPC history and concurrent snapshots.

Track logical state bytes, physical database bytes, history bytes, write amplification, snapshot bandwidth, compaction stalls and recovery time separately. A smaller file after unsafe pruning is not an optimization.

Detailed encodings/commit protocol: [14](14-block-and-state-commitment-spec.md). Availability/retention: [15](15-network-and-sync-protocol.md). Crash tests: [20](20-test-vectors-and-acceptance.md).

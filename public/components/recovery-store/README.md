# Storage interface implementation

This crate implements a bounded local RocksDB record-store interface for B0
dependency and durability spikes. It does not implement validator consensus,
authenticated snapshot import, public storage workers or complete EVM state.

`StoredBlockInput` and `StorageNetworkId` are explicitly untrusted storage inputs.
Every record binds network/genesis/bootstrap, parent/height and parent/block hash.
The caller must authenticate blocks and their execution/profile history before
using them in a production follower. Storing bytes never grants finality.

The fixed-width `EVESTR01` format is local storage metadata, not consensus RLP.
Opaque block payloads retain their externally selected serialization. Storage
keys and the durable record cursor are committed in one `WriteBatch`, with WAL
enabled and `WriteOptions::set_sync(true)`. The cursor describes a locally synced
record prefix, not an authenticated post-state or public recovery guarantee.
The supplied bootstrap anchor and complete execution inputs/checkpoint remain
the caller's responsibility; B1/B4 must integrate and verify those contracts.

One private DB handle owns a namespace; writes require exclusive Rust access.
No execution/query state lock is taken across I/O. API byte/count budgets are
enforced before sync; cache/write-buffer/background-job/file budgets are explicit.
RocksDB and OS allocations still have overhead; these knobs are not a strict
whole-process RAM cap or a zero-interference claim. B0 reports measured usage.

Physical checkpoints use the reviewed default flush path and are local recovery
artifacts, not authenticated network snapshots. Required tests cover malformed
storage bytes, wrong identity/parent/height, limits, atomic marker publication,
snapshot consistency, reopen/checkpoint recovery and abrupt process exit after
sync. Process-exit results do not establish power-loss durability.

Canonical owner: the public role's durable recovery-storage component. Master
archive and validator integration may reuse its narrow record-store contract;
this component contains no private master orchestration or voting authority.

Read plans [02](../../../docs/plan/02-state-and-storage.md),
[14](../../../docs/plan/14-block-and-state-commitment-spec.md),
[15](../../../docs/plan/15-network-and-sync-protocol.md),
[22](../../../docs/plan/22-code-layout-and-dependency-policy.md) and
[32](../../../docs/plan/32-regional-masters-and-public-persistence.md).

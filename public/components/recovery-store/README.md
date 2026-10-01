<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public durable recovery storage

Canonical owner: the public role's durable recovery-storage component. Validator
and master development/follower integrations may reuse its contracts. It owns no
private master orchestration, consensus finality, signing authority or route approval.

Read plans 02 (local-only: `../../../docs/plan/02-state-and-storage.md`),
14 (local-only: `../../../docs/plan/14-block-and-state-commitment-spec.md`),
15 (local-only: `../../../docs/plan/15-network-and-sync-protocol.md`),
16 (local-only: `../../../docs/plan/16-genesis-upgrade-and-recovery.md`),
22 (local-only: `../../../docs/plan/22-code-layout-and-dependency-policy.md`) and
32 (local-only: `../../../docs/plan/32-regional-masters-and-public-persistence.md`).

## Full-state repository — B1

`state/` implements one private RocksDB object per namespace, with an exclusive
write owner and read-only reader capabilities. One sync-enabled WAL `WriteBatch`
commits normalized accounts/slots/content-addressed code/system records, execution
hash history, canonical header/transactions/receipts/roots, the exact whole commit,
its canonical identity, and the local durable marker together. No payload segment
or required reference is acknowledged before successful sync. An ambiguous
write/sync or post-sync publication error fences fresh reads/writes until all
handles are dropped and the namespace is reopened/reconciled.

The canonical schema, complete-state validation, roots, content digest, header
checks and whole-commit RLP codec belong to `validator/components/state/`.
This package does not duplicate trie or commitment logic. A matching root is a
local consistency check, not execution validity or source-finality authentication.
`StateCommit` is an untrusted structural input; authenticated history and actual
execution validation remain upstream obligations. Durable acknowledgments expose
both the requested committed version and current store head; replay never lowers
that head. Replay requires identical complete canonical bytes, not merely height
or root equality.

Reopen verifies exact genesis/config/profile/schema, every retained parent and
whole-commit identity, all header/transaction/receipt/root references, code blobs
and complete current normalized rows. Historical hashes and the complete previous
256-block EVM lookup window are checked against retained execution headers through
canonical state validation. No pruning API exists; historic commits and code remain
recoverable even when current accounts/slots are deleted or recreated.

`StateService` owns the public query cache. It checks a complete immutable cached
view first, loads the repository only on a miss/stale version, then refreshes RAM.
Cache guards are dropped before database I/O. The separate commit owner does not
hold a cache/execution lock across writes or fsync. Read capture uses an actual
RocksDB snapshot sequence for metadata and both domains. It compares that sequence
with the published successful-sync version so an in-flight database write cannot
be called acknowledged durable. Old captured views stay internally consistent
across later commits; caller/runtime readiness and authenticated watermarks remain
separate B4 responsibilities.

Public APIs include `open_state_repository`, `commit_state`, `state_reader`,
`create_state_service`, `read_state_service`, `capture_state_snapshot`,
`read_snapshot_commit`, `export_state_snapshot` and `activate_snapshot_namespace`.
Immutable view getters are `commit()`/`sequence()`; snapshot getters are
`version()`/`sequence()`. No API exposes the raw DB handle.

Snapshot export uses one captured sequence and writes every exact canonical
recovery commit to new files with checksums/identities and explicit byte/count
limits. A complete manifest is written only after required files sync. Import
checks complete references, canonical content/parents, genesis and limits before
creating a NEW namespace; it never overwrites existing source/live data. A staged
namespace remains inactive while data syncs. Activation requires the full declared
head, then syncs the active marker. Interrupted/incomplete namespaces remain
preserved and ordinary startup rejects them; retry into another new namespace.
These local manifests do not authenticate a peer or imply source finality.

The development storage profile reuses canonical logical budgets, limits a physical
batch to 160 MiB, a complete export to 512 MiB, concurrent snapshots to 8 and
snapshot references to 16,384. Rocks cache/write-buffer/job/file settings are
explicit. These are local reference bounds, not consensus-invalidity rules, strict
whole-process RAM caps, mainnet capacity evidence or zero storage interference.
The initial baseline retains complete commits and duplicates narrow indices;
scaling/checkpoint compaction and asynchronous public workers remain later work.

Linux/WSL is the verified storage/snapshot platform. Directory fsync is implemented
on Unix; native Windows snapshot-durability acceptance is not claimed. Process
exit/termination tests do not prove hardware power-loss durability.

Run `cargo test --locked -p eve-storage` and strict Clippy, then the owning bulk
and structure gate. B1 tests include actual database reopen, exact replay versus
stale parents, cache refresh, sequence isolation, reference corruption and new
namespace snapshot activation. A real child-process exit bypasses destructors
before/after actual sync; separately labelled unit-only simulated failures test
fencing/lost acknowledgment. Those hooks cannot be selected in production
configuration. Structural I/O fixtures are labelled and do not claim that their
synthetic changes are execution-valid blocks; independent acceptance uses the
actual executor/genesis/fee/root paths.

## Narrow record store — retained B0 contract

`recovery/` remains a separate bounded opaque-record interface; it is not renamed
into full state. `StoredBlockInput` and `StorageNetworkId` are untrusted inputs.
The fixed-width `EVESTR01` metadata is local storage, not consensus RLP. Opaque
payloads retain their selected serialization; records/cursor use a sync-enabled
WAL batch. Its cursor is a locally synced prefix, not authenticated post-state.

Physical B0 checkpoints are local recovery artifacts. The complete public
storage worker, authenticated network import, master-offline peer recovery,
retention/chaos and secure-profile/mainnet throughput gates remain unfinished.
## Derived history index — B2

The auxiliary `EVEHISTORY01` schema is separately versioned from `EVESTATE01`.
It indexes execution hashes to heights, transaction hashes to exact height/index
locations and heights to bounded canonical state versions. It does not modify
canonical B1 commit bytes, identities, genesis, roots or consensus authority.

`ensure_history_index` owns an exclusive bounded bootstrap over retained recovery
commits. Each synced pass records the exact source commit identity and a whole-block
cursor. Count and physical batch bytes both bound progress, including the final
activation marker. A fitting prefix commits and resumes; a single block that cannot
fit fails explicitly. Interrupted passes reopen against the same canonical source.
Conflicting/duplicate identities, missing entries, orphan rows, stale cursors and
unsupported schemas fail rather than silently overwriting recovery data.

A complete index advances atomically with state, block data and the durable marker.
Auxiliary bootstrap writes publish their actual successful-sync sequence at the
same execution version. Ambiguous writes or lost acknowledgments fence fresh
reads/writes until reopen/reconciliation; unit-only fault hooks distinguish simulated
failures from actual hardware evidence. Already captured snapshots remain immutable.

`capture_history_snapshot` reserves a shared snapshot lease and checks the actual
database sequence against the acknowledged head and complete index. Narrow block
reads validate header, transaction/receipt encodings, roots, parent/version bindings
and retained identity references without loading a complete state at each height.
Account-state reads/proofs still use the complete captured-state path. Public raw
database access remains unavailable.

Reverse-lookup absence is authoritative only in that complete local index. Missing
or stale index coverage is `HISTORY_NOT_READY`, not an unknown null or invented
pruning result. This local consistency binding is not authenticated source finality.

Run the history_index/history_corruption/history_transactions integration cases
and the library's auxiliary failure/boundary cases, then the full storage/owning
bulk gate. Transaction fixtures include actual canonical execution and an explicitly
labelled structurally valid stale-nonce B1 recovery source used to reject duplicate
derived identities. They do not claim a valid second execution or validator finality.

Projection `maximum_block_bytes` includes all current/parent version, header,
transaction, receipt, root and commit-identity row value bytes; exact and one-byte
boundary cases verify this sum. It does not claim physical page-I/O or RSS accounting.
Auxiliary pass budgets cannot exceed repository physical batch bounds. Actual child
process exits after acknowledged partial/complete index passes verify no-destructor
reopen, exact prefix/cursor, idempotent completion and unchanged canonical bytes/head.

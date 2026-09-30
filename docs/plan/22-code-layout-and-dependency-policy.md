# 22 — Code layout, interfaces and dependency policy

## L01 — Proposed workspace

Create runtime roots only as their bulk requires them; current README files are not implementation. Mandatory nested organization, one-function files and the 200/400/600 physical-line rules are defined in [plan 25](25-folder-hierarchy-and-file-function-policy.md).

```text
master/                   eve-master follower runtime
public/                   eve-public RPC/P2P runtime
validator/                eve-validator application/consensus runtime
crates/
  primitives/             domain-separated IDs, addresses, quantities
  chain-spec/             validated genesis, parameters, fork schedules
  protocol/               versioned messages and canonical encodings
  evm/                    REVM adapter, serial reference, parallel executor
  state/                  immutable views, journals, commitments
  storage/                durable stores, snapshots, segments, retention
  consensus-interface/    verified consensus/commitment contracts
  consensus-comet/        baseline ABCI++ integration
  p2p/                    public distribution, sync client/server
  rpc/                    Ethereum-style RPC and simulation
  system/                 staking, fees, work, rewards, evidence
  observability/          bounded metrics, structured events
xtask/                    build/devnet/gates/packaging/bench orchestration
contracts/fixtures/       pinned Solidity development fixtures
integration/              TypeScript and multi-process scenarios
tests/                    golden/property/fault regression corpus
benchmarks/               repeatable generators and profiles
docs/                     specifications, runbooks and evidence summaries
```

This is a root map, not a flat source layout. Inside each runtime/crate, use `src/domain/capability/sub-capability/operation/.../function_name.rs`. The number of meaningful subfolder levels is not capped at three. Each behavioral file owns one primary function; shared logic belongs in a named domain crate, not copied into runtime folders.

Keep entry points thin: configuration, dependency construction, role-specific startup/shutdown. Do not create a generic `shared` crate containing unrelated business logic. Related code may share a crate initially, but must retain nested capability boundaries and single-function behavioral files. A small prototype is not permission to combine unrelated functions into a monolithic file.

## L02 — Baseline dependencies

Rust is the core language. REVM is the reference EVM integration. RocksDB is the initial persistent KV implementation behind replaceable storage traits. A CometBFT ABCI++ sidecar/adapter supplies the first real consensus implementation. Use maintained Ethereum type/RLP/cryptography crates with matching versions, and one pinned TypeScript client plus Solidity toolchain for developer fixtures.

These are development choices, not claims of maximum throughput or minimal disk size. Exact versions are deliberately selected through B0 compatibility spikes and then recorded as immutable pins; do not fill this document with guessed versions. Verify ABI/protobuf fields, EVM hooks, supported fork, root encoding and platform/toolchain support against the actual selected releases.

Linux is the reference deployment/test platform. Windows users can run the documented Linux/WSL development path; native Windows support is claimed only after its own CI/build/runtime evidence. Do not assume a package compiles on every platform because its language is cross-platform.

Do not introduce an ORM, SQL database, Kubernetes, message broker or paid external service into the consensus path without an evidenced requirement. Optional indexers/analytics are outside the first critical path.

## L03 — Shared contracts to define in B0/B1

`StateView`: immutable reads at one locally verified, finalized and applied height/root for accounts, code, storage and system records. A RAM-applied view does not imply local durability; callers receive the relevant watermarks and follow the readiness/lag policy.

`ExecutionEngine`: ordered transactions + deterministic block environment + parent view -> outcome containing journals, gas, receipts, logs and commitments. No live networking or clock dependency.

`StateStore`/`BlockStore`: atomic durable apply, sync-confirmed durable markers, consistent checkpoints/snapshots, recovery and retention through checked domain types. An explicit public recovery store may combine synced blocks and authentication/protocol/configuration/profile/validator history with periodic checkpoints; its replayable recovery watermark is distinct from checkpoint height and full state-store durable height. Enqueue completion and durable completion are distinct outcomes; incomplete replay coverage is not durable.

`StorageWorker`: bounded ordered immutable finalized batches -> local durable commit acknowledgment or explicit failure. One writer owns a local namespace; payload identity, parent/target heights and resource limits are frozen through B0/B1 compile-tested consumers. This is a public runtime/storage boundary, not an external broker or a dependency on master implementation.

`FinalityVerifier`: trusted genesis/checkpoint + headers/set transitions + consensus objects -> verified anchors and explicit authenticated heights. Returning a `VerifiedAnchor` must require the actual cryptographic checks, not a public constructor accepting raw hashes.

`SyncImporter`: staged snapshot/delta + VerifiedAnchor -> verified staged state -> atomic activation. It does not grant consensus authority to the data source.

`ConsensusAdapter`: drives proposal/validation/commit lifecycle, preserves engine safety semantics, and returns finalized ordered inputs with authenticated metadata.

`Signer`: engine-compatible canonical sign bytes + signing context -> signature under durable anti-equivocation/fencing rules. Persist necessary sign-state before releasing the signature; asynchronous public storage cannot replace this synchronous obligation.

`SystemModule`: metered/journaled user operations and deterministic block/epoch transitions with supply accounting.

The actual Rust signatures and serialization schemas are written alongside compile-tested consumers before interface freeze. No network-facing component may circumvent these contracts through direct database handles. Trait declarations are cohesive contract files; behavior implementations are decomposed into operation files, with only minimal delegation adapters as allowed by plan 25.

## L04 — Ownership and concurrency

One canonical state-publication owner and one durable commit owner per local database namespace. Execution workers receive immutable views/versioned overlays and return journals; they do not mutate committed global state concurrently. Public hot execution/query state is RAM-first by default, with finalized blocks and recoverable checkpoints persisted through an isolated asynchronous storage worker consuming ordered immutable batches. Reserve bounded handoff capacity before RAM application. Track read/write versions and deterministic retries. No execution/query state lock is held across disk writes, fsync, network waits, callbacks or unbounded simulation.

Bound storage queue bytes/count/oldest age and the lifetime of retained immutable buffers. Budget worker CPU, database/page cache, snapshots and compaction alongside execution/RPC work. B0 freezes versioned measured limits and readiness/lag policies; tests saturate declared limits and stall/fail sync. Apply backpressure before budgets are threatened. Storage isolation still consumes shared resources and carries no zero-overhead guarantee. Public may expose verified RAM-applied state within that policy while reporting distinct finalized/applied/durable/authenticated state heights under plan 12's H/H+1 binding; durable markers/acknowledgments require successful atomic sync and complete recovery coverage.

Recover a lost queued tail from authenticated durable peers after the last complete local commit. Never prune the sole recoverable finalized copy or require a master acknowledgment for transaction finality. Master NVMe persistence remains follower work; a RAM-only public mode is an explicit development/ephemeral alternative. See [plan 32](32-regional-masters-and-public-persistence.md) for the role and topology policy; these contracts are not implemented yet.

Bound every channel, cache and subscription. Cancellation must release resources without rolling back a committed height or losing signer safety. Shutdown stops new work, safely handles in-flight operations, flushes required durable state and records the last recoverable height.

Shared manifests, lockfiles, protocol types and database schemas have a single assigned owner during a multi-agent bulk. Integrate interface changes before dependent branches rather than resolving semantic conflicts at the end. Assign nested capability-folder ownership before parallel edits; the integrator coordinates module declarations and re-exports.

## L05 — Dependency acceptance

B0 records source URLs, version/tag/commit, artifact digest, license/security notes, supported platform and smoke-test output. Commit Cargo.lock and compiler/tool versions. Verify downloaded binary/container digests. Use official primary documentation for unfamiliar APIs. The maintained Ethereum execution specs/tests location must be checked; do not assume an archived fixture repository is current.

A dependency replacement requires an ADR, compatibility/migration impact, test evidence and benchmark comparison where performance is the reason. It may not weaken consensus, EVM, durability or public-build independence. Custom cryptography and from-scratch consensus are not an acceptable shortcut around integration work.

## L06 — Structural acceptance

Implement `cargo xtask check-structure` and its T-L01–T-L06 tests in B0 under plan 25. Run it in each bulk and on the integrated diff. Target 200 lines, review 201–400, require bounded exact-path exceptions for 401–600, and reject handwritten files over 600 physical lines. File length does not excuse multiple operations in one behavioral file. Module/facade, adapter, test and generated-file categories must be explicit and reviewed.

## Acceptance

Independent master/public/validator release builds; public-only package build without master source; compile-tested interfaces; deterministic serialization fixtures; lockfile reproducibility; no unchecked cross-runtime storage access; bounded shutdown/recovery; documented and tested platform claims; nested single-function organization and passing structure gates with no expired exceptions.

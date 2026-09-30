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

`StateView`: immutable reads at one committed height/root for accounts, code, storage and system records.

`ExecutionEngine`: ordered transactions + deterministic block environment + parent view -> outcome containing journals, gas, receipts, logs and commitments. No live networking or clock dependency.

`StateStore`/`BlockStore`: atomic apply, durable markers, consistent snapshots, recovery and retention through checked domain types.

`FinalityVerifier`: trusted genesis/checkpoint + headers/set transitions + consensus objects -> verified anchors and explicit authenticated heights. Returning a `VerifiedAnchor` must require the actual cryptographic checks, not a public constructor accepting raw hashes.

`SyncImporter`: staged snapshot/delta + VerifiedAnchor -> verified staged state -> atomic activation. It does not grant consensus authority to the data source.

`ConsensusAdapter`: drives proposal/validation/commit lifecycle, preserves engine safety semantics, and returns finalized ordered inputs with authenticated metadata.

`Signer`: engine-compatible canonical sign bytes + signing context -> signature under durable anti-equivocation/fencing rules.

`SystemModule`: metered/journaled user operations and deterministic block/epoch transitions with supply accounting.

The actual Rust signatures and serialization schemas are written alongside compile-tested consumers before interface freeze. No network-facing component may circumvent these contracts through direct database handles. Trait declarations are cohesive contract files; behavior implementations are decomposed into operation files, with only minimal delegation adapters as allowed by plan 25.

## L04 — Ownership and concurrency

One canonical application commit owner per local database namespace. Workers receive immutable views/versioned overlays and return journals; they do not mutate the committed global state concurrently. Track read/write versions and deterministic retries. Avoid locks held across network waits, fsync, callbacks or unbounded simulation.

Bound every channel, cache and subscription. Cancellation must release resources without rolling back a committed height or losing signer safety. Shutdown stops new work, safely handles in-flight operations, flushes required durable state and records the last recoverable height.

Shared manifests, lockfiles, protocol types and database schemas have a single assigned owner during a multi-agent bulk. Integrate interface changes before dependent branches rather than resolving semantic conflicts at the end. Assign nested capability-folder ownership before parallel edits; the integrator coordinates module declarations and re-exports.

## L05 — Dependency acceptance

B0 records source URLs, version/tag/commit, artifact digest, license/security notes, supported platform and smoke-test output. Commit Cargo.lock and compiler/tool versions. Verify downloaded binary/container digests. Use official primary documentation for unfamiliar APIs. The maintained Ethereum execution specs/tests location must be checked; do not assume an archived fixture repository is current.

A dependency replacement requires an ADR, compatibility/migration impact, test evidence and benchmark comparison where performance is the reason. It may not weaken consensus, EVM, durability or public-build independence. Custom cryptography and from-scratch consensus are not an acceptable shortcut around integration work.

## L06 — Structural acceptance

Implement `cargo xtask check-structure` and its T-L01–T-L06 tests in B0 under plan 25. Run it in each bulk and on the integrated diff. Target 200 lines, review 201–400, require bounded exact-path exceptions for 401–600, and reject handwritten files over 600 physical lines. File length does not excuse multiple operations in one behavioral file. Module/facade, adapter, test and generated-file categories must be explicit and reviewed.

## Acceptance

Independent master/public/validator release builds; public-only package build without master source; compile-tested interfaces; deterministic serialization fixtures; lockfile reproducibility; no unchecked cross-runtime storage access; bounded shutdown/recovery; documented and tested platform claims; nested single-function organization and passing structure gates with no expired exceptions.

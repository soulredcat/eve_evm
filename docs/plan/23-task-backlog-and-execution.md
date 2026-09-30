# 23 — Executable backlog and bulk delivery contract

## How to run this plan

This is the dependency-aware core implementation queue for `goal.md`. Run it together with mandatory [security queue 29](29-security-implementation-and-acceptance.md), which adds SEC0–SEC3 and T-M/T-P/T-BR requirements from plans 26–28. Every bulk includes design finalization, code, tests, integration, docs, evidence and review. Do not ask for permission after each completed bulk. Do not stop after scaffolding or a passing unit test while the required end-to-end path is missing.

The lead reads current Git state, STATUS and HANDOFF, chooses the ready bulk with the highest dependency value, assigns file ownership and records acceptance gates. Independent subtracks inside a bulk may run concurrently. Shared interfaces are resolved first. The integrator runs the complete gate after combining work.

Bulk states: NOT_STARTED, IN_PROGRESS, DONE, FAIL, BLOCKED_ENV, BLOCKED_INFRA, BLOCKED_OWNER, TARGET_UNMET. DONE requires all mandatory tasks and evidence. PASS/FAIL/NOT_RUN are test results, not interchangeable with project completion.

Every bulk also follows [plan 25](25-folder-hierarchy-and-file-function-policy.md): recursive domain/capability folders without a fixed depth limit, one primary function per behavioral file, and the 200/400/600 physical-line policy. Structure violations in affected code are repaired with the functional work rather than deferred to a final cleanup stage.

## Dependency map

```text
B0 -> B1 -> B2 -> B3 -> B4
                   |      |
                   +----> B5
B3+B4 -> B6
B2+B3+B4 -> B7
B4+B5+B6 -> B8
B6+B7+B8 -> B9
B9 -> B10 -> B11
```

B5 also requires the shared system interfaces from B1/B2. Research spikes may start earlier, but a dependent runtime bulk cannot be marked DONE before its prerequisites pass. Genuine external blockers do not stop unrelated ready work. SEC0 is part of B0; SEC1–SEC3 dependencies and secured-profile capacity gates are defined in plan 29.

## B0 — Pin the implementation contract

Tasks B0.1–B0.7:

1. Inspect checkout/toolchain/permissions; preserve user changes; create task-owned directories and a baseline report.
2. Select and pin compatible Rust/REVM/Ethereum types, RocksDB, CometBFT/ABCI binding, TypeScript client and Solidity compiler. Smoke-test execution, storage commit and the actual ABCI lifecycle. Run a bounded storage spike measuring batch/sync/checkpoint bytes, latency and resource use; freeze queue/lag/readiness and measurement contracts from plan 32 without requiring the later public runtime. Record official source revisions/digests and rejected alternatives.
3. Freeze compile-tested domain types, genesis/hash rules, system ABI/record encodings, native gas schedule, EVM environment and consensus/app-hash height mapping. Add worked vectors rather than unresolved TODOs.
4. Create workspace, task runner, gate manifest, CI baseline and dependency/test-corpus pins. The runner must fail if a requested test is absent.
5. Map R01–R12 to implementation tasks/tests and initialize real evidence tracking. Register T-N09/T-N10 for B4, T-N11 for B6 and T-N12 for B8 with concrete resource/crash/source/partition fixtures; later runtime tests remain NOT_RUN until implemented.
6. Implement `cargo xtask check-structure` with language-aware function-file classification, physical-line thresholds, reviewed exact-path exception/expiry handling and explicit generated/vendor exclusions. Add T-L01–T-L06, including 200/201/400/401/600/601 boundary fixtures and modules deeper than three levels. Wire the checker into CI and every bulk gate; it must check its own handwritten implementation and record warning rationales and exceptions.
7. Execute SEC0: inventory crypto/commitment/control paths; pin and test actual PQ implementations/integration surfaces; register T-M/T-P/T-BR tests and security gates; define two local EVE bridge identities. Do not pretend the initial Ed25519/secp256k1 engine is already PQ-secure.

Gate: actual build and interface smoke tests; byte vectors and development-config validation; bounded storage-spike evidence and versioned public persistence/readiness budgets; no guessed APIs or mutable dependency tags; passing structure-checker tests and an actual structure report; SEC0 outputs. `verify --bulk B0` is executable and runs `check-structure`. An absent/stubbed checker fails B0. Missing permissions/tools are documented blockers with exact alternatives, not fabricated success.

## B1 — Shared state and durable master harness

Tasks: implement StateView/journals and commitment interfaces; persistent state/block stores and atomic durable markers; snapshot staging; corruption detection/recovery; task-owned DEV_ALL_IN_ONE storage harness using shared components.

Gate: golden roots, insert/delete/account-code cases, crash-boundary replay, consistent snapshot and read-isolation tests. Repeated replay produces identical state. No consensus authority is added to MASTER_SYNC_ONLY.

## B2 — Serial EVM and usable RPC

Tasks: signed transaction decoding, Shanghai execution, canonical receipt/log/root generation, EVE fee hook and escrow credits; isolated simulation; public RPC and bounded nonce-aware mempool; pinned TypeScript/Solidity fixture through local development production of blocks.

Gate: T-E01–T-E06, relevant T-S/T-A cases, real deploy/native/ERC-20/swap/revert flows and restart checks. An unconditional RPC success response or hard-coded state fails the bulk. Distributed finality is not yet claimed. Legacy account compatibility is classical; SEC1 adds protected-account authorization without silently changing legacy signatures.

## B3 — Validator consensus

Tasks: integrate the pinned BFT adapter; four validator processes and proposer lifecycle; side-effect-free proposal execution; durable signing/application recovery; correct certificate, validator-set and H/H+1 commitment binding; quorum/partition tests.

Gate: T-C01–T-C10 with fixed genesis and separate keys, except later lifecycle scenarios use an authenticated test transition until B5 completes the user interface. Three-of-four progresses, two-of-four does not; no master dependency. Relevant signer/crash tests pass. Any temporary lifecycle test adapter is isolated from production and removed/replaced in B5. SEC1 subsequently enforces the actual hybrid consensus profile and majority-adversary tests; no post-finality wrapper is accepted as a substitute.

## B4 — Master follower and verified sync

Tasks: MASTER_SYNC_ONLY imports finalized data; source-independent header/validator verification; snapshot/delta export/import; resume/corruption handling; retained finalized data on validators; public RAM working state with local durable blocks/checkpoints; isolated ordered immutable storage batches and bounded resource/backlog controls; master-offline public recovery including authenticated missing-tail retrieval. Follow plan 32 without holding global RAM locks across IO or treating queued blocks as durable.

Gate: T-N01–T-N07, T-N09/T-N10, T-S04–T-S08, T-G04/T-G06 and master-offline catch-up. Delete public replica state and reconstruct it. Stop all masters while validators/public peers continue, inject public storage stalls and loss of unsynced RAM, then restart from complete local records and authenticated durable peer data. Assert configured queue/resource/lag limits, truthful durability/readiness and exact roots/receipts without asking master to decide blocks. Missing last-copy recovery data fails not-ready. Preserve authenticated profile/key-set history for later PQ and bridge clients.

## B5 — Staking, node work and fee-funded rewards

Tasks: native system ABI/escrows; validator/node registration and key possession; delegation, exit, unbonding and key rotation; deterministic epoch/set transitions; authenticated participation and assigned node-work receipts; commission/claims; evidence/slashing/jail; supply-conservation properties.

Gate: all T-V01–T-V10 plus real user-facing TypeScript staking/claim fixture. Epoch replay/claim/evidence deduplication must pass. The 40/30/30 split and documented dust policy conserve fees; no outside mint or manual master score is present. Replace B3-only lifecycle scaffolding with the actual production module. SEC1 covers activated hybrid key/control/recovery paths; SEC2 tests separate bridge backing conservation.

## B6 — Public network and distribution

Tasks: multi-peer transaction/data propagation; handshake/versioning; peer scoring/quotas; readiness and RPC history/proof behavior; eligible logical sync endpoints hiding internal master inventory; health/authenticated-freshness then service-RTT/throughput selection, bounded probes, hysteresis and fallback discovery/peers; public/validator package allowlists and reproducible source-package manifests; independent build without master implementation.

Gate: T-N08/T-N11, integrated T-N09/T-N10 under public network load, all T-A cases, T-Q04 and network/RPC overload tests. Ping or advertised height never grants trust to an invalid source or fresh-head eligibility to a lagging one. Valid older authenticated history remains usable for replay under its applicable height/profile. Increasing public nodes does not require a new privileged master connection for each; zone routing never changes voting authority. A clean extracted package can build and join the devnet. Activated profiles may not be silently downgraded during handshake or sync.

## B7 — Deterministic parallel execution

Tasks: versioned read/write tracking; worker scheduling/owned overlays; conflict detection/re-execution; ordered merge/commit; system-operation dependency tracking; serial fallback and performance counters.

Gate: every supported serial fixture and randomized workload yields identical state, system roots, receipts/logs, gas and header inputs in parallel. Include one hot pool, shared ERC-20/allowance, nonce contention, CREATE2 and atomic multi-pool revert. Report speedups and regressions honestly; correctness is mandatory even where contention prevents speedup. Re-run after secure-account and bridge operations are integrated.

## B8 — HA, upgrades, release safety and operations

Tasks: two independent master followers with private logical gateway routing and local storage fencing where applicable; verified failover/partition/catch-up runbooks and a capacity plan for ten replicas, without claiming ten deployed masters; protocol activation and schema migration; signed manifest staging/verification; voting-power-aware rolling upgrades; dependency/secret checks, fuzzing, metrics and diagnostic docs.

Gate: T-N12, integrated T-N10/T-N11 during master outages, all T-Q cases and T-G01–T-G08; injected bad releases/migrations/keys fail safely; master failover cannot sign; signer rollback cannot double-sign. Partition and recover two master stores, verifying identical authenticated history/roots at the same height. Independent master storage copies need not run a second consensus protocol. SEC3 adds integrated secure-profile migration, control-key and bridge incident drills.

## B9 — Regional behavior and DEVNET_ACCEPTED

Tasks: reproducible multi-role devnet deployment; simulated region delay/loss/partition scenarios; full developer/staking flows after outages; fresh-checkout packaging/build/run; requirement-to-evidence review and consolidated runbooks.

Gate: B0–B8 mandatory checks, including structure gates, remain green in the integrated revision; functional regional tests explicitly labelled simulated when local. All R01–R11 software gates pass. Publish a DEVNET_ACCEPTED report with the actual security profile and limitations. This checkpoint does not satisfy R12, SEC1–SEC3, external bridge readiness or authorize mainnet.

## B10 — Capacity program

Tasks: pinned W0–W6 generator profiles; real finalized-throughput instrumentation; sustained runs with state/IO/network accounting; concurrent public RAM application and durable storage-worker/checkpoint measurements; endpoint failover and per-master full-stream bandwidth/storage/catch-up accounting for one/two/estimated-ten layouts; profiling and bottleneck reports; incremental optimization in complete reviewed batches; deployment package for real multi-host/WAN testing.

Gate: reproducible baseline and each achieved target step with required evidence, predeclared persistence/queue/lag/readiness budgets, storage interference and post-run recovery results. Report actual durable bytes/s, queue slopes/age, fsync/compaction stalls, checkpoint memory/IO and per-copy replication costs; asynchronous storage is not zero overhead or proof of power-loss recovery. Continue 10k/50k/100k/250k/500k/1M attempts only within available/authorized resources. A failed target remains TARGET_UNMET; absent suitable machines are BLOCKED_INFRA. Do not lower the workload, fake finalized results or purchase resources automatically. Classical measurements are diagnostic; secured target acceptance requires SEC3 and all active authentication/proof costs included.

## B11 — Scaling validation and remaining target work

Tasks: resolve measured limiting components, evaluate alternative consensus/storage adapters where evidence warrants, implement/test necessary regional/partitioning experiments under plan 21, and rerun full correctness/recovery/regression gates after each architectural change.

Gate: SCALE_TARGET_VERIFIED only after declared mixed-EVM sustained 1M target, soak, bounded backlog, data availability, active security profile and recovery evidence. Sharding can be marked unnecessary only with evidence that the target is met without it; otherwise record experiments, unresolved constraints and next executable actions. No artificial claim that committing roots from independent conflicting states forms a valid global EVM chain. Bridge throughput and external route limitations are reported separately.

## Bulk integration protocol

Before closing a bulk: inspect the full diff; run format/lint/unit, `cargo xtask check-structure` and relevant integration/security gates; review unsafe/concurrency/authentication/recovery code; check documentation links and status accuracy; record evidence at the integrated revision; make a coherent local commit; update HANDOFF and proceed.

Record structural warnings with decomposition rationale, active reviewed exceptions and their split tasks. A handwritten file over 600 lines, expired exception, unrelated operations in a behavioral file or an absent mandatory checker blocks integration. No bulk may disable these checks to meet a functional or throughput target.

Do not force-push, overwrite unrelated edits, deploy production/live custody, restart unrelated services or bypass required approvals. Remote publishing follows current user authorization. Normal coding/build failures are work to fix, not reasons to ask the user to choose routine implementation details.

## Session and blocker handling

If a context/tool/quota limit interrupts work, record current branch/worktree, HEAD, dirty-diff summary, passing/failing commands, active task-owned process IDs, latest evidence and the exact next command. Do not mark incomplete work DONE or promise future background completion.

A blocker record contains task ID, category, exact failing action, evidence, attempted safe alternatives, smallest missing prerequisite and resume command. Continue independent tasks. Mainnet supply/governance/keys/audit/paid infrastructure and real external bridge deployment remain OWNER gates and are never silently resolved using development defaults. Keep stronger unmet majority-tolerance claims visible instead of redefining them as passing tests.

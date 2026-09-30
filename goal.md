# EVE EVM — implementation goal

## Objective

Turn this documentation-only repository into a working, tested EVM-compatible network with separate `master/`, `public/` and `validator/` runtimes, then prove **1,000,000 aggregate finalized TPS as a mainnet release requirement**. Preserve a versioned, measured development path toward future 100M and potentially 1B TPS as technology advances; these future targets are not current capability claims. Do not stop after producing another plan, scaffold, mocked RPC response, in-memory demo, or compiling workspace.

User decisions are authoritative: validators make consensus decisions; developer-operated masters synchronize and persist finalized results; public nodes are permissionless; runtime sources are separate; fees split 40% burn, 30% node rewards and 30% validator rewards; useful work and availability determine reward eligibility. Organize implementation in meaningful recursively nested folders, with one primary function per behavioral file and the 200/400/600 physical-line policy in plan 25.

The owner additionally requires majority-attack resilience, post-quantum security and a secure bridge. Implement the explicit fault models, authenticated security profiles, local bridge fixture and acceptance gates in plans 26–29. Do not claim unlimited 51% immunity or end-to-end quantum security from the existing classical baseline.

EVE must also be bridge-friendly to Ethereum, Solana and other chains through the adapter architecture in plan 30. Ethereum and Solana are mandatory named two-way integration targets, not optional examples. Preserve an EVM core while implementing separate EVM-contract and Solana-program endpoints, typed assets/addresses, source verification, SDK/transaction builders and resumable transfers. The two-EVE bridge fixture is only the first step. Other routes are extensible, not automatically supported or approved.

## Current scope boundary

Follow plan 31. Liquidity provision, a production DEX, reserve-funded trading and automatic pool-price stabilization are deferred to a separate future module. Do not implement them in this goal or make core launch depend on that module. Keep ordinary Solidity/AMM test fixtures and swap workloads as EVM compatibility and performance tests; bridge custody/accounting remains mandatory. The previously discussed reserve/multi-admin lock proposal is recorded separately and does not authorize automatic trading, real issuance or reserve withdrawal.

## Authority and reading order

1. Current explicit user instructions and repository safety boundaries.
2. `docs/plan/24-decision-register.md` and `docs/plan/31-mainnet-target-and-module-boundaries.md`: current architectural decisions, release target, scope and superseded assumptions.
3. Specifications 12–22, policies 25–28, and plan 30, with their acceptance tests. Plans 26–28 refine the older classical development baseline; they do not retroactively make it secure.
4. `docs/plan/23-task-backlog-and-execution.md`, `docs/plan/29-security-implementation-and-acceptance.md`, and plan 30's INT0–INT3 queue: mandatory core, security and interoperability work.
5. Overviews 00–11, which summarize rather than override those specifications.
6. `docs/execution/STATUS.md`, `EVIDENCE.md`, and `HANDOFF.md`: actual progress.

For an inconsistency, preserve safety and the latest architectural decision, add a decision record, repair affected documents/tests together, then continue. Do not silently broaden scope, reintroduce master-controlled finality or add the deferred liquidity module.

## Execution contract

Work autonomously through every ready core bulk B0–B11, security bulk SEC0–SEC3 and interoperability bulk INT0–INT3 using `AGENTS.md`. SEC0 and INT0 are included in B0; other dependencies are in plans 29/30. Plan by dependencies and implement complete vertical slices. Research an unfamiliar API using primary documentation, make a bounded spike, record the result, then implement. Do not spend the whole run repeatedly rewriting plans.

Use five specialist responsibilities when supported: lead/orchestrator; protocol/EVM; state/network/performance; correctness/security; reviewer/integrator. Independent tasks may run in parallel; overlapping files and consensus interfaces require explicit ownership. Tests and review are part of each bulk, not a final cleanup stage. Assign Ethereum and Solana adapter ownership separately after shared interface agreement; do not create duplicated custody/replay logic.

Each bulk must leave a usable repository state. Run its gates, repair failures, save evidence, update status, create a coherent local commit, and proceed. For a genuine external blocker, record the failed command, cause, required resource and resume action, and work on other ready tasks. Lack of certainty is a reason for a test or source check, not for inventing a guarantee.

## Mandatory code organization

Use `runtime-or-crate/src/domain/capability/sub-capability/operation/.../function_name.rs`. Nest as deeply as the actual responsibility requires; neither stop at three levels nor create empty directories merely for depth. One production behavioral file owns one primary function. Extract other operations/helpers into correctly named files; keep types, facades and thin delegation adapters in the explicit categories of plan 25.

Target at most 200 formatted physical lines per file; 201–400 requires decomposition review and a retained-size rationale; 401–600 requires a reviewed exact-path temporary exception with an expiring split task; more than 600 is a hard failure for handwritten files. Line counts include imports, comments, blank lines and tests. No minification, blanket exclusions or lowered acceptance criteria.

B0 must implement and test `cargo xtask check-structure`, covering T-L01–T-L06, and wire it into CI and every bulk gate. Preserve deterministic behavior and package independence during refactors. Do not postpone folder/file cleanup until the project ends, and do not claim the structure checker exists before it is actually implemented. Apply explicit language coverage to bridge Solidity, Solana Rust and TypeScript SDK code as those modules are added.

## Required software outcomes

The implementation must provide:

- A pinned, reproducible Rust workspace and executable development workflow.
- Deterministic REVM execution, signed transaction validation, real gas accounting, receipts, logs and authenticated state roots.
- Persistent state, crash recovery and restart-safe validator signing records.
- Public Ethereum-style JSON-RPC and P2P ingestion with bounded queues, rate limits and documented compatibility differences.
- Four-validator development consensus, proposer rotation, independently verified proposals, quorum finality, validator-set transitions and no master override.
- A protected master follower that can stop, return and catch up without deciding finality or being acknowledged on the transaction hot path.
- Verified snapshot/delta bootstrap from multiple sources, corruption rejection, restart recovery, pruning safety and retained data during master outages.
- Staking, delegation, node work accounting, validator participation, the 40/30/30 fee ledger, slashing evidence, jailing and unbonding with supply-conservation tests.
- Separate public/validator packages that build without master code; signed release verification and operator-controlled rolling updates.
- Deterministic parallel execution proven equivalent to the serial oracle, including hot-pool contention and atomic multi-contract transactions.
- Master replication/failover, partition tests, multi-region single-chain deployment and an explicit experimental path for state partitioning.
- Reproducible workload generators, telemetry, fault injection, capacity reports and measured release-candidate evidence for the 1M target.
- Function-focused, recursively nested implementation modules, passing structural checks and no expired size exceptions.
- Majority-adversary tests and honest fault-assumption reporting, without automatic quorum reduction or master takeover.
- Actual mandatory hybrid/PQ authentication through consensus, protected accounts, clients, control/recovery and release paths; crypto/commitment inventory and tested migrations.
- A two-EVE-devnet bridge with fake assets, authenticated finality/inclusion, replay protection, conserved backing, bounded exposure and tested incident handling.
- Ethereum/EVE and Solana/EVE adapters and real destination endpoints with two-way fake-asset acceptance, exact decimals/asset identity, wallet/SDK examples and per-direction verification evidence under plan 30.
- An extensible route registry that exposes capabilities, trust/PQ coverage, unsupported-token reasons, fees, progress and approval state without pretending every chain is already integrated.

A master-only early prototype composes the same shared execution/storage components with a local test producer. It must not become a second production consensus implementation. A production-mode master with all validators offline preserves data and reports stalled finality; it does not invent blocks.

## Completion levels

### DEVNET_ACCEPTED

All B0–B9 mandatory acceptance checks pass. A clean checkout can build and start a local network, deploy a Solidity fixture, send native/ERC-20 transfers, execute successful and reverting swaps, read accurate receipts/logs, restart every role, lose one validator, lose the master, recover a public replica, settle rewards, and verify unchanged roots and balances. Packaging, structure, security, upgrade and chaos tests must also pass. Disclose whether this checkpoint is CLASSICAL_DEV or a stronger profile. It is not completion of security, external interoperability or throughput targets, and is not permission to launch mainnet. Test-fixture swaps do not require implementing the deferred production liquidity module.

### SECURITY_PROFILE_ACCEPTED

SEC0–SEC3 and T-M/T-P/T-BR acceptance pass for the named fault model, protected authentication scope and local bridge fixture. Publish crypto coverage, residual classical/external dependencies and migration/incident evidence. Classical-only consensus, post-hoc PQ stamps, an unprotected account-recovery path or a mocked bridge cannot satisfy this level.

CometBFT's baseline does not guarantee safety/liveness under 51% Byzantine power. Any stronger continuity requirement remains `UNSATISFIED_BY_BASELINE` until a reviewed alternative with explicit assumptions meets it. Passing an attack test does not prove universal immunity. An external bridge route remains disabled until its own adapter and authorization gates pass.

### INTEROP_DEV_ACCEPTED

INT0–INT3 and T-I01–T-I12 in plan 30 pass for both named external targets and both directions under the declared local/test trust profile. Ethereum and Solana endpoint/application tests, authenticated source verification, exact accounting, SDK signing/fees/resume and incident tests must exist. A simulator-only contract test, fake verifier, or two-EVE result cannot close these targets. Record each direction and external dependency separately.

This checkpoint neither approves live routes nor certifies Ethereum/Solana as post-quantum. Preserve secure EVE verification while clearly classifying external classical or added-trust dependencies. If a required verifier or approved trust model is unavailable, its target remains blocked/incomplete; continue the other implementable adapter and SDK work.

### SCALE_TARGET_VERIFIED

B10–B11 evidence proves the declared sustained 1M finalized TPS workload under the rules in plans 08, 20, 21 and 27–31. Include raw data, active security profile, workload mix, latency, real cross-node network/storage behavior, bounded backlog, recovery and independent replay checks. Report transfer, mixed-contract, hot-pool, cross-domain and bridge results separately. A projected sum of workers, compressed roots, ingress rate, classical-only benchmark or short burst is not proof of the secured target. EVE TPS does not imply the same completed bridge TPS or remove external-chain finality delays.

For mainnet readiness, evidence must cover the release candidate and production-representative configuration/topology. Do not describe a prelaunch capacity benchmark as an already-observed live-mainnet load. Future 100M/1B goals require separate measured evidence; preserve extensibility without claiming those capacities or expanding the first-launch threshold beyond 1M.

If software runs but measurements fall short, status is TARGET_UNMET and the next optimization remains work. If suitable hardware is unavailable, status is BLOCKED_INFRA for the affected experiment. Neither state may be renamed DONE. Continue all implementable work and produce the exact deploy/run package needed to resume.

### MAINNET_READY

Requires SCALE_TARGET_VERIFIED at the 1M release threshold above, explicit owner-approved genesis/economics/governance, independent security review, operational/key ceremony, licensed distribution decisions, production infrastructure and route-specific bridge approval. This goal does not authorize deployment or spending. Code completion does not certify economic security, PQ strength of every dependency, majority immunity or audit status. The separately deferred liquidity/price-stabilization module is not a prerequisite for core mainnet readiness.

## Definition of done per requirement

A requirement is DONE only with real implementation, positive and negative tests, integration with adjacent modules, updated operator/developer documentation, passing relevant gates including structure checks, and recorded evidence at the reviewed commit. Stubs, TODO bodies, unconditional success, fake signatures, hard-coded roots, ignored recovery errors and disabled assertions do not qualify.

`docs/execution/STATUS.md` tracks every core/security/interoperability bulk and unmet stronger target. `docs/execution/EVIDENCE.md` links the actual reports. Large logs stay in an artifact directory with checksums rather than being dumped into Git. `docs/execution/HANDOFF.md` always identifies the next uncompleted action so a new session can continue without this conversation.

## Permission and resource boundaries

Operate on this repository and task-created development resources. Use fake development tokens only. Do not access unrelated wallets, keys, databases or bots. Do not purchase infrastructure, change GitHub visibility, choose a binding source license, use real signing keys, issue real tokens or deploy mainnet/live custody. Keep paid review and external route validation requirements explicit. No added signer/provider trust or external production route is silently authorized by the request for bridge compatibility. Do not fund reserves, select real custody arrangements or add automatic reserve spending from the earlier economic proposal.

Session/tool limits must produce a resumable checkpoint, not a promise of unattended future work. A goal file guides an available Codex session; it does not remove runtime, quota, permission or hardware limits.

## First actions

Inspect the checkout and toolchain. Read the plans, latest scope in plan 31, and status. Execute B0 including SEC0 and INT0: pin and smoke-test dependency/consensus/crypto and chain-adapter interfaces, finalize byte-level fixtures and security/route inventories, build the requirement-to-test matrix, create core/security/interoperability gates, and implement the structure checker with boundary tests. Then proceed through the next ready bulks without asking for another planning round.

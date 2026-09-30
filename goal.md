# EVE EVM — implementation goal

## Objective

Turn this documentation-only repository into a working, tested EVM-compatible network with separate `master/`, `public/` and `validator/` runtimes, then execute the capacity and regional-scaling program toward **1,000,000 aggregate finalized TPS**. Do not stop after producing another plan, scaffold, mocked RPC response, in-memory demo, or compiling workspace.

User decisions are authoritative: validators make consensus decisions; developer-operated masters synchronize and persist finalized results; public nodes are permissionless; runtime sources are separate; fees split 40% burn, 30% node rewards and 30% validator rewards; useful work and availability determine reward eligibility. Organize implementation in meaningful recursively nested folders, with one primary function per behavioral file and the 200/400/600 physical-line policy in plan 25.

The owner additionally requires majority-attack resilience, post-quantum security and a secure bridge. Implement the explicit fault models, authenticated security profiles, local bridge fixture and acceptance gates in plans 26–29. Do not claim unlimited 51% immunity or end-to-end quantum security from the existing classical baseline.

## Authority and reading order

1. Current explicit user instructions and repository safety boundaries.
2. `docs/plan/24-decision-register.md`: current architectural decisions and superseded assumptions.
3. Specifications 12–22, policies 25–28, and their acceptance tests. Plans 26–28 refine the older classical development baseline; they do not retroactively make it secure.
4. `docs/plan/23-task-backlog-and-execution.md` and `docs/plan/29-security-implementation-and-acceptance.md`: mandatory core and security queues.
5. Overviews 00–11, which summarize rather than override those specifications.
6. `docs/execution/STATUS.md`, `EVIDENCE.md`, and `HANDOFF.md`: actual progress.

For an inconsistency, preserve safety and the latest architectural decision, add a decision record, repair affected documents/tests together, then continue. Do not silently broaden scope or reintroduce master-controlled finality.

## Execution contract

Work autonomously through every ready core bulk B0–B11 and security bulk SEC0–SEC3 using `AGENTS.md`. SEC0 is part of B0; other dependencies are in plan 29. Plan by dependencies and implement complete vertical slices. Research an unfamiliar API using primary documentation, make a bounded spike, record the result, then implement. Do not spend the whole run repeatedly rewriting plans.

Use five specialist responsibilities when supported: lead/orchestrator; protocol/EVM; state/network/performance; correctness/security; reviewer/integrator. Independent tasks may run in parallel; overlapping files and consensus interfaces require explicit ownership. Tests and review are part of each bulk, not a final cleanup stage.

Each bulk must leave a usable repository state. Run its gates, repair failures, save evidence, update status, create a coherent local commit, and proceed. For a genuine external blocker, record the failed command, cause, required resource and resume action, and work on other ready tasks. Lack of certainty is a reason for a test or source check, not for inventing a guarantee.

## Mandatory code organization

Use `runtime-or-crate/src/domain/capability/sub-capability/operation/.../function_name.rs`. Nest as deeply as the actual responsibility requires; neither stop at three levels nor create empty directories merely for depth. One production behavioral file owns one primary function. Extract other operations/helpers into correctly named files; keep types, facades and thin delegation adapters in the explicit categories of plan 25.

Target at most 200 formatted physical lines per file; 201–400 requires decomposition review and a retained-size rationale; 401–600 requires a reviewed exact-path temporary exception with an expiring split task; more than 600 is a hard failure for handwritten files. Line counts include imports, comments, blank lines and tests. No minification, blanket exclusions or lowered acceptance criteria.

B0 must implement and test `cargo xtask check-structure`, covering T-L01–T-L06, and wire it into CI and every bulk gate. Preserve deterministic behavior and package independence during refactors. Do not postpone folder/file cleanup until the project ends, and do not claim the structure checker exists before it is actually implemented.

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
- Reproducible workload generators, telemetry, fault injection, capacity reports and a measured path to the throughput target.
- Function-focused, recursively nested implementation modules, passing structural checks and no expired size exceptions.
- Majority-adversary tests and honest fault-assumption reporting, without automatic quorum reduction or master takeover.
- Actual mandatory hybrid/PQ authentication through consensus, protected accounts, clients, control/recovery and release paths; crypto/commitment inventory and tested migrations.
- A two-EVE-devnet bridge with fake assets, authenticated finality/inclusion, replay protection, conserved backing, bounded exposure and tested incident handling.

A master-only early prototype composes the same shared execution/storage components with a local test producer. It must not become a second production consensus implementation. A production-mode master with all validators offline preserves data and reports stalled finality; it does not invent blocks.

## Completion levels

### DEVNET_ACCEPTED

All B0–B9 mandatory acceptance checks pass. A clean checkout can build and start a local network, deploy a Solidity fixture, send native/ERC-20 transfers, execute successful and reverting swaps, read accurate receipts/logs, restart every role, lose one validator, lose the master, recover a public replica, settle rewards, and verify unchanged roots and balances. Packaging, structure, security, upgrade and chaos tests must also pass. Disclose whether this checkpoint is CLASSICAL_DEV or a stronger profile. It is not completion of the new security requirements, the throughput target or permission to launch mainnet.

### SECURITY_PROFILE_ACCEPTED

SEC0–SEC3 and T-M/T-P/T-BR acceptance pass for the named fault model, protected authentication scope and local bridge fixture. Publish crypto coverage, residual classical/external dependencies and migration/incident evidence. Classical-only consensus, post-hoc PQ stamps, an unprotected account-recovery path or a mocked bridge cannot satisfy this level.

CometBFT's baseline does not guarantee safety/liveness under 51% Byzantine power. Any stronger continuity requirement remains `UNSATISFIED_BY_BASELINE` until a reviewed alternative with explicit assumptions meets it. Passing an attack test does not prove universal immunity. An external bridge route remains disabled until its own adapter and authorization gates pass.

### SCALE_TARGET_VERIFIED

B10–B11 evidence proves the declared sustained 1M finalized TPS workload under the rules in plans 08, 20, 21 and 27–29. Include raw data, active security profile, workload mix, latency, real cross-node network/storage behavior, bounded backlog, recovery and independent replay checks. Report transfer, mixed-contract, hot-pool, cross-domain and bridge results separately. A projected sum of workers, compressed roots, ingress rate, classical-only benchmark or short burst is not proof of the secured target.

If software runs but measurements fall short, status is TARGET_UNMET and the next optimization remains work. If suitable hardware is unavailable, status is BLOCKED_INFRA for the affected experiment. Neither state may be renamed DONE. Continue all implementable work and produce the exact deploy/run package needed to resume.

### MAINNET_READY

Requires explicit owner-approved genesis/economics/governance, independent security review, operational/key ceremony, licensed distribution decisions, production infrastructure and route-specific bridge approval. This goal does not authorize deployment or spending. Code completion does not certify economic security, PQ strength of every dependency, majority immunity or audit status.

## Definition of done per requirement

A requirement is DONE only with real implementation, positive and negative tests, integration with adjacent modules, updated operator/developer documentation, passing relevant gates including structure checks, and recorded evidence at the reviewed commit. Stubs, TODO bodies, unconditional success, fake signatures, hard-coded roots, ignored recovery errors and disabled assertions do not qualify.

`docs/execution/STATUS.md` tracks every core/security bulk and unmet stronger target. `docs/execution/EVIDENCE.md` links the actual reports. Large logs stay in an artifact directory with checksums rather than being dumped into Git. `docs/execution/HANDOFF.md` always identifies the next uncompleted action so a new session can continue without this conversation.

## Permission and resource boundaries

Operate on this repository and task-created development resources. Use fake development tokens only. Do not access unrelated wallets, keys, databases or bots. Do not purchase infrastructure, change GitHub visibility, choose a binding source license, use real signing keys, issue real tokens or deploy mainnet/live custody. Keep paid review and external route validation requirements explicit.

Session/tool limits must produce a resumable checkpoint, not a promise of unattended future work. A goal file guides an available Codex session; it does not remove runtime, quota, permission or hardware limits.

## First actions

Inspect the checkout and toolchain. Read the plans and status. Execute B0 including SEC0: pin and smoke-test dependency/consensus/crypto interfaces, finalize byte-level test fixtures and the security inventory, build the requirement-to-test matrix, create the task runner/security gates, and implement its structure checker with boundary tests. Then proceed through the next ready core/security bulks without asking for another planning round.

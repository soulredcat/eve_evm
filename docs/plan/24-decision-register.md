# 24 — Current decisions and bounded defaults

Revision: 2026-09-30, including the owner's recursive folder/function-file requirement. This register resolves contradictory exploratory suggestions in the earlier conversation. The initial documentation baseline was commit `1d8b73c7eec223a1460c9e09cfb2d18a5627dee6`.

## Accepted user direction

| ID | Decision |
|---|---|
| D01 | Public/validator network decides canonical ordering/finality. Master synchronizes/persists; it cannot override quorum. |
| D02 | Separate `master/`, `public/`, `validator/` roots with shared reusable protocol/execution modules. |
| D03 | Anyone may run public nodes; no fixed protocol-wide count. Voting eligibility/power is separately staked and activated. |
| D04 | Master infrastructure is developer-operated and protected. This does not make all source in a public repository private. |
| D05 | RAM-heavy working state and compact durable storage are goals. Validators still need durable signing/recovery data. |
| D06 | Fees are 40% burn, 30% node pool and 30% validator pool; work/availability are authenticated, not self-reported. |
| D07 | Bulk snapshot/delta synchronization; no direct public access to master databases. |
| D08 | Regional ingress and later scaling retain deterministic global semantics and explicit finality. |
| D09 | Main performance objective remains 1M aggregate finalized TPS, measured end to end rather than assumed. |
| D10 | Build incrementally from a local all-in-one/master-first harness to separated production roles. |
| D11 | State synchronization and signed/operator-controlled software update are separate systems. |
| D12 | Codex implements dependency-aware bulks with tests/review/evidence and continues all unblocked work. |
| D13 | Source follows clear domain/capability/operation subfolders recursively; there is no fixed three-level or other arbitrary depth cap. Create meaningful modules, not empty decorative layers. |
| D14 | One production behavioral file owns one primary function/operation. Other behavioral helpers get separate files; non-behavioral and minimal trait-adapter categories are explicit in plan 25. |
| D15 | File-size policy is target 200 physical lines, decomposition review for 201–400, reviewed temporary exception for 401–600, and hard failure above 600 for handwritten files. B0 implements the checker; every bulk enforces it. |

## Superseded assumptions

- Master is not the unilateral source of consensus truth and does not finalize during quorum loss.
- Public RPC count is not validator voting power, and additional full validators do not automatically multiply TPS.
- A matching delta/root or master signature is not a validity proof.
- Validator working state may be memory-heavy; validator signing safety may not be RAM-only.
- Country IDs and random pool identities do not resolve conflicting shared-state writes.
- Bulk synchronization amortizes messages, not all data bytes or WAN finality latency.
- MASTER_ONLY is a development composition, not an alternate production trust mode.
- The database engine/NVMe generation alone is not evidence of compactness or throughput.
- A short file or a small prototype is not permission to bundle multiple production operations or bypass the recursive folder policy.

## Selected development baseline

These defaults allow implementation to proceed without repeated design questions. They are not binding mainnet allocations or claims that this is the final 1M-TPS architecture.

| Area | Baseline | Change rule |
|---|---|---|
| Core | Rust workspace | Evidence-backed ADR for material replacement |
| EVM | REVM, Shanghai execution, protected legacy/types 1 and 2 | Explicit fork/version tests |
| Consensus | CometBFT ABCI++ adapter, four local equal-power validators | Reviewed adapter, safety/binding tests |
| State | Ethereum-compatible EVM trie plus authenticated system trie | Preserve commitments or schedule migration |
| Storage | RocksDB/WAL behind traits | Benchmark and recovery comparison before replacement |
| Dev identity | EVM chain ID 31337; `eve-local-v1`; immutable genesis hash | Separate genesis for another network |
| Resource limits | 30M gas/block, 4 MiB full consensus block, 128 KiB raw tx | Versioned config and capacity evidence |
| Pricing | EIP-1559-style pricing, initial dev base fee 1 gwei, floor 1 base unit | Explicit config/compatibility tests |
| Fee allocation | 4000/3000/3000 basis points; validator pool gets block split dust | Owner direction preserved |
| Epoch/set | 1000 blocks; up to 64 active; backed stake power | Deterministic activation with adapter delay |
| Self-bond | Validator 10,000 dev tokens; rewarded node 100; 18 decimals | Development parameters only |
| Commission | Default 10%, max 20%, delayed changes | Development parameters only |
| Unbonding | At least 7 consensus-time days and 2000 blocks | Must cover evidence/trust windows |
| Evidence | Engine age rules with dev limits 1000 blocks/24 hours | Pin exact engine behavior and vectors in B0 |
| Checkpoint | Trusted anchor period 24 hours | Validate against unbond/evidence security assumptions |
| Retention | At least 10k blocks or 24h, preserving more; no unsafe last-copy pruning | Cover evidence/recovery limits |
| Releases | Two-of-three development release signatures, operator policy | Owner-controlled production trust roots |
| Platform | Linux reference; WSL documented where applicable | Other support requires evidence |
| Code organization | Plan 25 recursive capability folders and one-function behavioral files | Structural tests and reviewed bounded exceptions; no bypass of D13–D15 |

Genesis hash is the immutable network/spec digest. Runtime configuration digests and upgrade versions may change only through their declared rules; they must not be substituted for network identity during verification. Header extraData uses the immutable genesis-domain digest defined by plan 14, not an unannounced local config hash.

## B0 must finalize through executable spikes

Pin exact compatible dependency/tool versions and source/artifact digests. Freeze byte-level structures, native system ABI/gas schedule, reserved escrows, genesis fixtures, app-hash/validator activation mapping and the task-runner commands. Implement `cargo xtask check-structure`, its versioned policy/exclusion manifests and T-L01–T-L06 tests from plan 25. These are bounded implementation tasks with specified outcomes, not permission to leave indefinite TBD sections.

When a baseline library cannot satisfy a requirement, record the actual failing interface/test and select a compatible maintained alternative through an ADR. Do not invent a new security model simply to make a demo run.

## Owner gates

Mainnet chain ID/name and supply allocation; real validator economics/governance; key ceremony and release trust roots; binding source licenses/private repository changes; external security audit; purchased infrastructure; mainnet launch; Alephium bridge/settlement or other custody integration. None is authorized by fake development tokens or example parameters.

## Decision change template

Record ID/date, affected requirements, old/new rule, reason/evidence, alternatives, compatibility/migration impact, safety/capacity tests, operator consequences and approval scope. Update every affected plan/test/config in the same bulk. Safety exceptions cannot be hidden as performance optimizations.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 24 — Current decisions and bounded defaults

Revision: 2026-10-01, including D40's EVE-ownchain priority and testnet-timed deferral of separate bridge/adapter programs. Earlier bridge/interoperability scope and verified B0 evidence remain historical facts where superseded. This register resolves contradictory exploratory suggestions in the earlier conversation. The initial documentation baseline was commit `1d8b73c7eec223a1460c9e09cfb2d18a5627dee6`.

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
| D09 | Mainnet release requires proven 1M aggregate finalized TPS with the active security profile and release-representative evidence, not just a long-term aspiration or ingress benchmark. |
| D10 | Build incrementally from a local all-in-one/master-first harness to separated production roles. |
| D11 | State synchronization and signed/operator-controlled software update are separate systems. |
| D12 | Codex implements dependency-aware bulks with tests/review/evidence and continues all unblocked work. |
| D13 | Source follows clear domain/capability/operation subfolders recursively; there is no fixed three-level or other arbitrary depth cap. Create meaningful modules, not empty decorative layers. |
| D14 | One production behavioral file owns one primary function/operation. Other behavioral helpers get separate files; non-behavioral and minimal trait-adapter categories are explicit in plan 25. |
| D15 | File-size policy is target 200 physical lines, decomposition review for 201–400, reviewed temporary exception for 401–600, and hard failure above 600 for handwritten files. B0 implements the checker; every bulk enforces it. |
| D16 | Majority-attack resilience is mandatory and evaluated under explicit adversary assumptions. The existing BFT baseline does not guarantee safety/liveness at 51% Byzantine power; stronger unmet requirements remain visible rather than being renamed complete. |
| D17 | Post-quantum security is a required implementation/migration track, not a label for the classical baseline. Enforce the named crypto profile throughout consensus, protected accounts, clients, control/recovery and releases. |
| D18 | Any later bridge program requires authenticated source finality/inclusion, replay prevention, backing conservation, exposure controls and route-specific crypto/trust assessment. No relayer/master assertion authorizes value movement. Implementation timing and core scope follow D40. |
| D19 | Earlier scope made B0–B11, SEC0–SEC3 and INT0–INT3 mandatory. D40 supersedes its bridge/interop requirement: current mandatory work is B0–B11 plus SEC0/SEC1/SEC3; SEC2 and INT0–INT3 are deferred separate programs. Historical tests are not erased. |
| D20 | Secure-profile 1M TPS must be measured with required authentication enabled. A classical-only result is diagnostic, not secured-target completion. |
| D21 | Earlier Ethereum <-> EVE and Solana <-> EVE targets are retained future candidates under D40. Separate adapter programs adapt to EVE only after EVE testnet and subsequent owner scope; EVE core need not know those chains. |
| D22 | Keep EVE as an EVM core. Future Solana adapters use its actual program/account/token/signing semantics. Future bridge interfaces preserve per-chain finality, address/asset identity and explicit trust/PQ capabilities outside core; D40 controls timing. |
| D23 | Any separately authorized bridge program needs SDK/quotes/status/retry and round-trip tests for its declared scope. Adding an RPC URL, passing two-EVE tests, or deploying escrow cannot mark a route supported or approved. These are deferred program gates under D40. |
| D24 | Production liquidity/DEX, automated pool-price stabilization and reserve-funded trading are deferred to a separately developed future module. Do not build them in the current core goal or make core completion depend on them. |
| D25 | Retain an evidence-driven architectural path toward future 100M and potentially 1B finalized TPS as technology advances. These are not proven capacities or additional first-launch gates. |
| D26 | The USD-100M/100M-base-token backing proposal, possible later collateral and multiple-admin unlock requirement remain separately recorded in plan 31. No actual funding, mint, fixed market price, unilateral master release or automatic trading is authorized. |
| D27 | Root `local-tests/` is machine-local storage for exploratory tests, debugging and raw output. Never stage, commit, force-add or push it. Required reproducible tests and sanitized fixtures remain versioned for collaborators. |
| D28 | GitHub must contain clean, reviewed source and documentation in English. Apply the publication/index/outgoing-history checks in `CONTRIBUTING.md`; exclude local artifacts, secrets and personal configuration. This policy does not itself authorize a push. |
| D29 | Default public nodes keep working state in RAM and finalized block/recovery checkpoints durable. An isolated ordered storage worker uses bounded queues/resource budgets; durable acknowledgement requires actual successful sync, not enqueue or OS page cache. Storage overhead must be measured. |
| D30 | Public restart with masters offline recovers the complete local durable prefix and authenticates any missing tail from durable peers. Validators retain independent recovery data and durably record anti-double-sign state before releasing votes. Never prune the last recovery copy. |
| D31 | One master can serve multiple public nodes through regional endpoints/relays. Start development with one master/one public and the existing validator baseline; prepare two independently verified master replicas and a planned ten-master regional topology, subject to existing production approval gates. |
| D32 | Public uses one preferred nearby eligible sync endpoint without requiring private master inventory. Proof/network/profile eligibility and verified freshness precede service-latency ranking; fallback discovery and independent transaction P2P remain available. Hidden topology is not a security guarantee. |
| D33 | `zone_id` is operational routing/placement metadata distinct from network/genesis, chain ID and voting power. It does not establish shard ownership, authorize conflicting state writes or multiply finalized TPS. Actual partitioning follows plan 21. |
| D34 | Role ownership/file placement is an absolute integration rule: public/master/validator behavior stays in its role root, reusable operations stay in named domain crates, each behavioral file owns one clearly named operation, and public/validator builds have no direct or transitive private-master dependency. Misplacement or mixed responsibilities cannot be waived for a build/performance pass. |
| D35 | Do not use a root `crates/` or `create/` directory. Named reusable components live under the canonical owning role's `components/` subtree; current recovery storage belongs to public, execution/authentication to validator. Preserve one canonical implementation and reproducible distribution copies. |
| D36 | The master host may explicitly compose all roles with separate configuration/credentials and unchanged finality authority. Copy-ready public and validator role directories must each build/run alone with all dependencies, lockfile/toolchain and sanitized examples included, without the original repository or private master access. A monorepo-only build cannot satisfy standalone acceptance. |
| D37 | The verified B0 contract uses exact Rust/REVM/RocksDB/Alloy/Comet/protobuf/client/compiler pins recorded in versioned manifests. Comet v0.40.0 source retains its upstream 0.39.0 version string, identified by the exact revision suffix. Its native key oneof cannot enforce classical AND PQ for one voter; retain only labelled classical development and fail activated hybrid startup until the reviewed SEC1 engine integration exists. No post-finality wrapper closes that gap. |
| D38 | B0 freezes development genesis/header/system/ABI/gas byte vectors and public resource/readiness/source policy under canonical role ownership. Mainnet genesis is refused. Public caller verification flags are metadata, not authenticated capabilities; the small storage spike and declared 452 MiB/512 MiB pool budget do not prove runtime enforcement, power-loss resilience or capacity. B4/B6 retain those measurements and enforcement gates. |
| D39 | Historical B0 verification included SEC0 and INT0 under the then-active scope. Exact nonzero core test discovery, structure, ownership, format, strict lint, release and source/config/evidence identity remain mandatory. D40 removes SEC2/INT0–INT3 from current core selection; retain their IDs/history as deferred. Missing current core coverage fails rather than being omitted. Local foundation acceptance is separate from hosted CI, complete roles, security acceptance and throughput. |
| D40 | EVE's own chain is the current goal. Bridge SEC2/T-BR and external interoperability INT0–INT3/T-I, including the two-EVE bridge fixture, are `DEFERRED_UNTIL_EVE_TESTNET` for separately authorized programs. Reaching testnet permits later scope consideration, not automatic development. Adapters conform to EVE through versioned public interfaces and ordinary authenticated transactions; core has no external-chain route/client/custody dependency or remote latency, block-production, voting, finality or durable-acknowledgement dependency. Preserve EVM/RPC semantics, validator/public/master security/recovery, 40/30/30 and the secured 1M target. |

## Superseded assumptions

- Master is not the unilateral source of consensus truth and does not finalize during quorum loss.
- Public RPC count is not validator voting power, and additional full validators do not automatically multiply TPS.
- A matching delta/root or master signature is not a validity proof.
- Validator working state may be memory-heavy; validator signing safety may not be RAM-only.
- Default public RAM working state does not imply RAM-only finalized history. Asynchronous storage cannot promise zero interference, unbounded buffering, or durability before sync.
- Country IDs and random pool identities do not resolve conflicting shared-state writes.
- Bulk synchronization amortizes messages, not all data bytes or WAN finality latency.
- MASTER_ONLY is a development composition, not an alternate production trust mode.
- The database engine/NVMe generation alone is not evidence of compactness or throughput.
- A short file or a small prototype is not permission to bundle multiple production operations or bypass the recursive folder policy.
- A greater-than-two-thirds quorum is not unconditional 51%-attack immunity; stronger cryptography does not stop a malicious legitimate quorum.
- Classical EVM accounts/consensus, a PQ library, TLS, or a PQ stamp ignored by consensus verification do not establish end-to-end quantum security.
- A bridge cannot repair failed endpoint assumptions merely by adding more relayers, waiting a fixed delay or signing messages with PQ keys.
- EVM compatibility does not automatically integrate every EVM chain, execute Solana programs or supply external finality proofs. External dependencies do not become PQ-secure by connecting to EVE.
- Earlier mandatory external integration and two-EVE bridge work does not survive D40 as a current core prerequisite. Its deferral is an explicit owner scope change, not a passed test, failed core requirement or permission to start a replacement program now.
- Earlier reserve/DEX stabilization suggestions are not authorization to add a liquidity controller to the core; the owner deferred that application. Existing AMM tests are not a production DEX requirement.
- 1M TPS is not optional for the stated mainnet release target. Conversely, the future 100M/1B direction is not a capability already demonstrated by this documentation.

## Selected development baseline

These defaults permit incremental core implementation. They are not binding mainnet allocations, proof of the final 1M-TPS architecture, or completed majority/PQ security. Plans 26–32 explicitly extend or narrow this classical starting point; deferred bridge specifications do not authorize current adapter implementation.

| Area | Baseline | Change rule |
|---|---|---|
| Core | Rust workspace | Evidence-backed ADR for material replacement |
| EVM | REVM, Shanghai execution, protected legacy/types 1 and 2 | Classical compatibility profile; protected PQ accounts require plan 27 |
| Consensus | CometBFT ABCI++ adapter, four local equal-power validators | Classical starting point; actual hybrid enforcement/engine integration under plans 26–27 |
| State | Ethereum-compatible EVM trie plus authenticated system trie | Preserve commitments or schedule migration; analyze quantum strength before whole-chain claims |
| Storage | RocksDB/WAL behind traits | Benchmark and recovery comparison before replacement |
| Public persistence | RAM working state plus durable finalized recovery blocks/checkpoints through bounded worker | Plan 32; separate applied/durable/authenticated watermarks and test storage interference/restart |
| Regional masters | One development master, verified two-replica evolution, planned ten-master layout | Each master follows validator finality; nearby eligible endpoint selection/failover; no launch authorization |
| Dev identity | EVM chain ID 31337; `eve-local-v1`; immutable genesis hash | Separate genesis for another network |
| Resource limits | 30M gas/block, 4 MiB full consensus block, 128 KiB raw tx | Versioned config; include actual PQ/proof byte and CPU costs |
| Pricing | EIP-1559-style pricing, initial dev base fee 1 gwei, floor 1 base unit | Explicit config/compatibility tests |
| Fee allocation | 4000/3000/3000 basis points; validator pool gets block split dust | Owner direction preserved |
| Epoch/set | 1000 blocks; up to 64 active; backed stake power | Deterministic activation with adapter delay |
| Self-bond | Validator 10,000 dev tokens; rewarded node 100; 18 decimals | Development parameters only |
| Commission | Default 10%, max 20%, delayed changes | Development parameters only |
| Unbonding | At least 7 consensus-time days and 2000 blocks | Must cover evidence/trust windows |
| Evidence | Engine age rules with dev limits 1000 blocks/24 hours | Pin exact engine behavior and vectors in B0 |
| Checkpoint | Trusted anchor period 24 hours | Validate against unbond/evidence security and crypto migration assumptions |
| Retention | At least 10k blocks or 24h, preserving more; no unsafe last-copy pruning | Cover evidence/recovery limits |
| Releases | Two-of-three development release identities, operator policy | Each counted identity meets the active signature profile; production roots need owner approval |
| Platform | Linux reference; WSL documented where applicable | Other support requires evidence |
| Code organization | Plan 25 recursive capability folders and one-function behavioral files | Structural tests and reviewed bounded exceptions; no bypass of D13–D15 |
| PQ experiments | Reviewed ML-DSA-65 integration; classical AND PQ for activated hybrid authority | SEC0 pins actual support/vectors; no unsupported production security claim |
| Bridge foundation | Historical two-EVE fake-asset reservation only; SEC2 deferred | `DEFERRED_UNTIL_EVE_TESTNET`; later separate scope and real consensus/proof evidence required |
| External integration candidates | Ethereum/Solana and other separately scoped adapter programs under plan 30 | `DEFERRED_UNTIL_EVE_TESTNET`; subsequent owner scope, direction-specific evidence and custody approval required |
| Application boundary | No production liquidity/DEX or automated stabilizer in this core goal | Future separate module under new scope; keep existing EVM/AMM fixtures |
| Mainnet capacity | Proven 1M finalized TPS release gate | Active profile, production-representative evidence; no silent threshold reduction |

Genesis hash is the immutable network/spec digest. Runtime configuration digests and upgrade versions may change only through their declared rules; they must not be substituted for network identity during verification. Header extraData uses the immutable genesis-domain digest defined by plan 14, not an unannounced local config hash.

## B0 must finalize through executable spikes

Pin exact compatible dependency/tool versions and source/artifact digests. Freeze byte-level structures, native system ABI/gas schedule, reserved escrows, genesis fixtures, app-hash/validator activation mapping and task-runner commands. Implement `cargo xtask check-structure`, its versioned policy/exclusion manifests and T-L01–T-L06 tests from plan 25.

Include current SEC0 from plan 29: core crypto/commitment inventory, actual PQ/consensus integration spikes and T-M/T-P test registration. Historical B0 also implemented INT0 metadata, source/tool research and bridge identity reservations; preserve the evidence at its actual revision. Those components, T-BR and T-I are no longer mandatory core deliverables under D40. Read plan 31 before selecting work: do not create adapter, liquidity/stabilization or reserve-spending scaffolds. Current core tasks have specified outcomes, not permission to leave indefinite TBD sections.

Include plan 32's source-selection/readiness/storage budgets, typed watermarks, recovery contracts and T-N09–T-N12 registration. B0 performs bounded compile-tested/interface spikes; distributed runtime acceptance belongs to B4/B6/B8, not a fabricated B0 pass.

When a baseline library cannot satisfy a requirement, record the actual failing interface/test and select a compatible maintained alternative or reviewed extension through an ADR. Do not invent a new security model or certify an unsupported property simply to make a demo run.

## Owner gates

Mainnet chain ID/name and supply allocation; real validator economics/governance; key ceremony and release trust roots; binding source licenses/private repository changes; independent security audit; purchased infrastructure; mainnet launch; later bridge/adapter program scope after EVE testnet; real-value bridge or settlement deployment and route limits. Any subsequently authorized local bridge tests use fake assets only. Development parameters and EVE testnet readiness do not authorize production custody. Reserve funding/custody, issuance, exact admin threshold and the future liquidity module require their own explicit specification and authorization.

Changes adding external trust, exceptional fork/social recovery or weaker crypto/fault guarantees require explicit disclosure and approval; they cannot masquerade as routine performance tuning or bridge compatibility. No interoperability-provider partnership or issuer-native token support is presumed.

## D40 change record — 2026-10-01

Owner instruction replaces the earlier mandatory bridge/interoperability queue with EVE-ownchain-first delivery. External development is eligible only after EVE reaches testnet, in separately scoped programs that adapt to EVE. Requiring full mainnet/1M completion before considering that scope would exceed the clarified timing. Automatic adapter development at testnet would exceed the authorization.

Affected scope: D18–D23/D39, plans 23/28–31, core security acceptance, bulk registry/pins and role package ownership. No consensus, crypto profile, Shanghai fork rule, fee rule or native proof format is changed. Historical B0/SEC0/INT0 commands/results remain valid for their recorded revision; current gates select only current core requirements and expose deferred IDs explicitly. Verification must confirm no external adapter dependency in production execution, block production, voting, finality or durable acknowledgement. Deferred program safety rules remain in plans 28/30; real custody remains unapproved.

## D41 — Classical development genesis anchor

B3's initial Comet AppHash is the existing canonical genesis StateVersion content
digest, binding its identity and complete state before the first certified header.
Do not construct ApplicationCommitment(0); genesis keeps that field absent, and
positive execution heights retain the unchanged EVE_APP_V1 commitment. This closes
the startup boundary without changing the EVM genesis/header/fee encodings.
Compatibility and independent genesis/header/replay vectors are mandatory before
B3 integration. Mainnet genesis remains an explicit owner boundary.

## D42 — Native Ed25519 verification parity

The pinned Comet verifier uses ZIP-215. Dalek's strict or uncofactored verification
cannot silently replace that consensus behavior. B3 pins ed25519-zebra 4.2.0 for
individual native classical certificate verification, with published ZIP-215 rules
and exact native edge vectors. Existing account/hybrid authentication and signing
rules remain unchanged. New dependency/source/license/audit checks are mandatory;
this is not a security certification or PQ activation. Unsupported profiles fail.

## D43 — Genesis-bound B3 acceptance lifecycle

Plan 23's temporary authenticated transition uses a disabled-by-default
`development-acceptance` feature, a bounded manifest bound in the canonical
genesis fixture contract's code, and real signed EVM calls/successful receipts.
Rotation retains its existing owner/backed power; leave/jail are bounded native
test-set removals. Native updates retain H+2 activation and separate H+1/H+3
metadata. Normal builds reject marked acceptance genesis, preventing a silent
rule difference at the same genesis identity. Exact replay derives the same
updates from retained canonical transactions/receipts. No arbitrary operator or
master schedule triggers changes. This does not implement production staking,
slashing or key ceremonies; B5 must replace the temporary adapter. The separate
bounded malformed-proposal hook never bypasses actual non-nil vote execution or
durable signing. Full T-C05/T-C07 acceptance is mandatory before B3 integration.

## D44 — EVE classical enrollment point policy

Initial and acceptance-future EVE enrollments require a canonical nonidentity
prime-order Ed25519 public point. Reuse existing pinned Dalek `to_edwards`,
`is_torsion_free` and canonical compression; no curve primitive or dependency
upgrade is introduced. Small-order rejection alone does not reject mixed-order
points. The native generic ZIP-215 verifier and raw-byte validator hash stay
unchanged; preserved mixed-order native vectors must still verify there while
EVE enrollment rejects them. Generated existing EVE keys/encodings remain valid.
This development admission hardening is not PQ activation or certification.
Primary API contracts: [Dalek 2.2.0](https://docs.rs/ed25519-dalek/2.2.0/ed25519_dalek/struct.VerifyingKey.html)
and [curve25519-dalek 4.1.3](https://docs.rs/curve25519-dalek/4.1.3/curve25519_dalek/edwards/struct.EdwardsPoint.html#method.is_torsion_free).

## Decision change template

Record ID/date, affected requirements, old/new rule, reason/evidence, alternatives, compatibility/migration impact, safety/capacity tests, operator consequences and approval scope. Update every affected plan/test/config in the same bulk. Safety exceptions cannot be hidden as performance optimizations.

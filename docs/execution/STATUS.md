# Implementation status

Updated: 2026-10-01. **B0 including SEC0/INT0 passed its complete local foundation gate: 205 tests, format, strict lint, structure and release build.** Complete role runtimes, standalone distributions, devnet/security/bridge/capacity acceptance remain unachieved.

## Overall

- Documentation/goal package: prepared for implementation, including plans 00–32.
- Collaboration policy: English shared prose and clean GitHub publication rules recorded; ignored `local-tests/` workspace established. Publication checks are recorded in [PUB-20260930-01](EVIDENCE.md#pub-20260930-01--local-test-isolation-and-publication-policy); no implementation bulk or runtime gate is completed by this hygiene task.
- Regional/public persistence contract: [plan 32](../plan/32-regional-masters-and-public-persistence.md) records 1→2→10 independent master followers, preferred eligible nearby sync endpoints, operational zone IDs, RAM working state with isolated bounded durable recovery storage, and T-N09–T-N12. Documentation verification is recorded in [DOC-20260930-02](EVIDENCE.md#doc-20260930-02--regional-masters-and-public-persistence); all four new runtime cases are `NOT_IMPLEMENTED` / `NOT_RUN`.
- Runtime foundations: nine role/tool packages now cover execution, authentication, the exact consensus API, recovery storage, public resource/readiness/source policy, public interop metadata and validator protocol/bridge contracts. Root `crates/` is absent. Complete role runtimes and standalone copy/build/run acceptance remain unimplemented.
- Integrated foundation gate: 205 cases pass, zero failed/ignored/filtered; format, strict workspace Clippy and release build pass. Structure covers 569 files, 15 exact exclusions, zero violations/warnings. T-L01–T-L06 have 64 cases; provisioning has 6 and verification 16. The complete 20-bulk registry, R01–R12 and 48 security/interop/public-persistence registrations preserve full requirements without claiming future runtime acceptance. [B0 evidence](B0-20261001.md) records source/config identities and reproduction.
- Tool/API verification: fresh task-local pinned Go/Comet/OpenSSL/Solidity/Node and locked TypeScript/viem provisioning passed, including actual compiler/client probes and receipt reuse. The native engine's unsupported hybrid path fails closed. Patched OpenSSL replaces the retired PQClean cross-check wrappers; official NIST coverage is retained.
- Dependency/publication review: current audit reports zero known vulnerabilities and two retained unmaintained warnings (derivative/paste). No existing locked package was upgraded. The first hosted CI run failed during locale-dependent archive inspection before its gate; [CI repair evidence](CI-20261001.md) records the reproduction, strict pin correction and 87 passing scoped cases. The additional regression raises the catalog to 206; a hosted/full-gate result is recorded only after it actually occurs.
- DEVNET_ACCEPTED: NOT_ACHIEVED.
- CONSENSUS_RESILIENCE_TESTED: NOT_ACHIEVED.
- PQ_PROFILE_VERIFIED: NOT_ACHIEVED.
- BRIDGE_DEVNET_ACCEPTED: NOT_ACHIEVED.
- SECURITY_PROFILE_ACCEPTED: NOT_ACHIEVED.
- INTEROP_DEV_ACCEPTED: NOT_ACHIEVED.
- SCALE_TARGET_VERIFIED: NOT_ACHIEVED.
- 51_PERCENT_CONTINUITY: UNSATISFIED_BY_BASELINE; no unconditional majority-tolerance guarantee.
- EXTERNAL_BRIDGE_ROUTES: DISABLED_NOT_APPROVED.
- MAINNET_READY: NOT_AUTHORIZED / NOT_ASSESSED.
- Full runtime/security-profile/majority/interop/bridge/capacity acceptance and benchmarks: NOT_RUN / NOT_ACHIEVED. Foundation structure/interface results are recorded separately from runnable distribution and runtime acceptance.

## Core bulk ledger

| Bulk | Status | Next acceptance requirement | Evidence |
|---|---|---|---|
| B0 | DONE | Complete local foundation gate passed; continue B1 | [205-case acceptance](B0-20261001.md) |
| B1 | NOT_STARTED | State/store/master harness and crash recovery | None |
| B2 | NOT_STARTED | Real serial EVM/fees/RPC/developer fixture | None |
| B3 | NOT_STARTED | Four-validator classical baseline consensus and signing safety; not a PQ claim | None |
| B4 | NOT_STARTED | Verified follower/snapshot/delta/retention; isolated public persistence and master-offline restart T-N09/T-N10 | None |
| B5 | NOT_STARTED | Staking/work/rewards/slashing invariants | None |
| B6 | NOT_STARTED | P2P/independent public/validator packages; eligible nearby endpoint discovery/failover T-N11 | None |
| B7 | NOT_STARTED | Serial/parallel equivalence and profiling | None |
| B8 | NOT_STARTED | HA/upgrades/security/operations; independent two-master partition/catch-up T-N12 | None |
| B9 | NOT_STARTED | Integrated/regional devnet acceptance with declared security profile | None |
| B10 | NOT_STARTED | Sustained capacity program; secure-profile evidence required for secured target | None |
| B11 | NOT_STARTED | Scaling experiments and verified secured 1M target | None |

## Security bulk ledger

| Bulk | Status | Next acceptance requirement | Evidence |
|---|---|---|---|
| SEC0 | DONE | Primitive/inventory/integration boundary accepted; SEC1 enforcement remains required | [B0 including SEC0](B0-20261001.md) |
| SEC1 | NOT_STARTED | Actual consensus/account/client/recovery hybrid enforcement, majority tests and migration | None |
| SEC2 | NOT_STARTED | Two-EVE-devnet bridge, proofs, conserved backing, replay/incident and hybrid tests | None |
| SEC3 | NOT_STARTED | Integrated security profile, recovery/release drills, secure capacity handoff | None |

## Interoperability bulk ledger

| Bulk | Status | Next acceptance requirement | Evidence |
|---|---|---|---|
| INT0 | DONE | Metadata/capability/tool/feasibility contracts accepted; all custody disabled and INT1–INT3 remain pending | [B0 including INT0](B0-20261001.md) |
| INT1 | NOT_STARTED | Real Ethereum contracts/Solana program, chain adapters and SDK local fixtures | None |
| INT2 | NOT_STARTED | Per-direction authenticated source and destination verification, negative tests and resource measurements | None |
| INT3 | NOT_STARTED | Fresh-checkout two-way named integration acceptance, SDK/recovery and route matrix | None |

## Named route targets

| Direction | Implementation | Source/destination verification | Live approval |
|---|---|---|---|
| Ethereum -> EVE | NOT_STARTED | NOT_IMPLEMENTED | DISABLED_NOT_APPROVED |
| EVE -> Ethereum | NOT_STARTED | NOT_IMPLEMENTED | DISABLED_NOT_APPROVED |
| Solana -> EVE | NOT_STARTED | NOT_IMPLEMENTED | DISABLED_NOT_APPROVED |
| EVE -> Solana | NOT_STARTED | NOT_IMPLEMENTED | DISABLED_NOT_APPROVED |
| Additional chains, including Alephium | EXTENSIBLE_NOT_INTEGRATED | ROUTE_SPEC_REQUIRED | DISABLED_NOT_APPROVED |

ETHEREUM_INTEROP_DEV and SOLANA_INTEROP_DEV are NOT_ACHIEVED. Actual route trust mode and crypto coverage are not yet established by implementation. No EVE PQ requirement certifies an external endpoint.

Dependencies are in [plan 23](../plan/23-task-backlog-and-execution.md), [plan 29](../plan/29-security-implementation-and-acceptance.md), and [plan 30](../plan/30-cross-chain-interoperability.md). B0 including SEC0 and INT0 is first. Do not mark downstream work DONE because a directory, README, test name, adapter interface or crypto wrapper exists.

## Claim boundaries

The starting CometBFT/Ed25519 and secp256k1 EVM profile is classical. Plans 26–30 add requirements, not completed protections or integrations. Under its stated fault model the baseline does not guarantee correct continued finality with 51% Byzantine power. Local bridge acceptance does not approve an external chain or real custody. Two-EVE acceptance does not complete the named Ethereum/Solana integrations, and application simulator results do not prove source finality.

## Update rules

Every DONE row must link reviewed implementation and passing evidence at the actual integrated revision. Keep failed/blocked targets and stronger unmet security requirements visible. Genuine blockers record category, cause, attempted alternatives and exact resume action; they do not prevent independent work.

Use [EVIDENCE.md](EVIDENCE.md) for run records and [HANDOFF.md](HANDOFF.md) for resumption. Keep R01–R12, T-L, T-M/T-P/T-BR and T-I01–T-I12 requirements traceable. Do not equate documentation completion, functional devnet acceptance, security-profile acceptance, interoperability acceptance and mainnet readiness.

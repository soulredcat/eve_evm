# Implementation status

Updated: 2026-09-30. The current specifications include Ethereum/Solana interoperability and regional masters/RAM-first public persistence. **No runtime implementation or execution gate has passed yet.**

## Overall

- Documentation/goal package: prepared for implementation, including plans 00–32.
- Collaboration policy: English shared prose and clean GitHub publication rules recorded; ignored `local-tests/` workspace established. Publication checks are recorded in [PUB-20260930-01](EVIDENCE.md#pub-20260930-01--local-test-isolation-and-publication-policy); no implementation bulk or runtime gate is completed by this hygiene task.
- Regional/public persistence contract: [plan 32](../plan/32-regional-masters-and-public-persistence.md) records 1→2→10 independent master followers, preferred eligible nearby sync endpoints, operational zone IDs, RAM working state with isolated bounded durable recovery storage, and T-N09–T-N12. Documentation verification is recorded in [DOC-20260930-02](EVIDENCE.md#doc-20260930-02--regional-masters-and-public-persistence); all four new runtime cases are `NOT_IMPLEMENTED` / `NOT_RUN`.
- Runtime code: not started.
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
- Runtime, structure, cryptographic, attack, interoperability and bridge tests/benchmarks: NOT_RUN.

## Core bulk ledger

| Bulk | Status | Next acceptance requirement | Evidence |
|---|---|---|---|
| B0 | NOT_STARTED | Pin dependencies/interfaces; workspace/xtask/vectors/structure checker; SEC0 and INT0 inventories | None |
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
| SEC0 | NOT_STARTED | B0 crypto/commitment inventory, real integration spikes, attack/proof test registration | None |
| SEC1 | NOT_STARTED | Actual consensus/account/client/recovery hybrid enforcement, majority tests and migration | None |
| SEC2 | NOT_STARTED | Two-EVE-devnet bridge, proofs, conserved backing, replay/incident and hybrid tests | None |
| SEC3 | NOT_STARTED | Integrated security profile, recovery/release drills, secure capacity handoff | None |

## Interoperability bulk ledger

| Bulk | Status | Next acceptance requirement | Evidence |
|---|---|---|---|
| INT0 | NOT_STARTED | B0 chain/asset/route interfaces; pinned Ethereum/Solana feasibility, fixtures and T-I gates | None |
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

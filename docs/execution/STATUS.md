# Implementation status

Updated: 2026-09-30. This revision adds majority-adversary, post-quantum and bridge specifications plus the mandatory security execution queue. **No runtime implementation or execution gate has passed yet.**

## Overall

- Documentation/goal package: prepared for implementation, now including plans 26–29.
- Runtime code: not started.
- DEVNET_ACCEPTED: NOT_ACHIEVED.
- CONSENSUS_RESILIENCE_TESTED: NOT_ACHIEVED.
- PQ_PROFILE_VERIFIED: NOT_ACHIEVED.
- BRIDGE_DEVNET_ACCEPTED: NOT_ACHIEVED.
- SECURITY_PROFILE_ACCEPTED: NOT_ACHIEVED.
- SCALE_TARGET_VERIFIED: NOT_ACHIEVED.
- 51_PERCENT_CONTINUITY: UNSATISFIED_BY_BASELINE; no unconditional majority-tolerance guarantee.
- EXTERNAL_BRIDGE_ROUTES: DISABLED_NOT_APPROVED.
- MAINNET_READY: NOT_AUTHORIZED / NOT_ASSESSED.
- Runtime, structure, cryptographic, attack and bridge tests/benchmarks: NOT_RUN.

## Core bulk ledger

| Bulk | Status | Next acceptance requirement | Evidence |
|---|---|---|---|
| B0 | NOT_STARTED | Pin dependencies, smoke-test interfaces, create workspace/xtask/vectors/structure checker and SEC0 inventory | None |
| B1 | NOT_STARTED | State/store/master harness and crash recovery | None |
| B2 | NOT_STARTED | Real serial EVM/fees/RPC/developer fixture | None |
| B3 | NOT_STARTED | Four-validator classical baseline consensus and signing safety; not a PQ claim | None |
| B4 | NOT_STARTED | Verified follower/snapshot/delta/retention | None |
| B5 | NOT_STARTED | Staking/work/rewards/slashing invariants | None |
| B6 | NOT_STARTED | P2P and independent public/validator packages | None |
| B7 | NOT_STARTED | Serial/parallel equivalence and profiling | None |
| B8 | NOT_STARTED | HA, upgrades, security and operations | None |
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

Dependencies are in [plan 23](../plan/23-task-backlog-and-execution.md) and [plan 29](../plan/29-security-implementation-and-acceptance.md). B0 including SEC0 is first. Do not mark downstream work DONE because a directory, README, test name or crypto wrapper exists.

## Claim boundaries

The starting CometBFT/Ed25519 and secp256k1 EVM profile is classical. Plans 26–29 add requirements, not completed protections. Under its stated fault model the baseline does not guarantee correct continued finality with 51% Byzantine power. Local bridge acceptance does not approve an external chain or real custody.

## Update rules

Every DONE row must link reviewed implementation and passing evidence at the actual integrated revision. Keep failed/blocked targets and stronger unmet security requirements visible. Genuine blockers record category, cause, attempted alternatives and exact resume action; they do not prevent independent work.

Use [EVIDENCE.md](EVIDENCE.md) for run records and [HANDOFF.md](HANDOFF.md) for resumption. Keep R01–R12, T-L and the added T-M/T-P/T-BR requirements traceable. Do not equate documentation completion, functional devnet acceptance, security-profile acceptance and mainnet readiness.

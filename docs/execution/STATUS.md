<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Implementation status

Updated: 2026-10-01. B0/SEC0 foundations and B1 have complete passing local gates.
The latest B1 repair also passes hosted CI with 273 cases. B2's complete local
gate passes 348 cases, including ownership and D40 scope. [Hosted B2](CI-20261001-B2.md)
also passes 348 at b5f98df; that separately observed run is not B3 acceptance.

## Current owner scope

Development is PAUSED_BY_OWNER during incomplete B3. The owner authorizes a
current-state checkpoint branch and draft PR to main, without merge, followed
by stopping work. The PC runs other programs. New owner input is required before
further development/checks; B4 remains NOT_STARTED. Read [the checkpoint](B3-checkpoint-20261001.md).

EVE develops its own chain first. [D40](../plan/24-decision-register.md) defers
SEC2/INT0–INT3, bridge and external-chain programs until EVE testnet and a later
separate-program instruction. Such programs adapt to EVE and do not govern
core transaction/voting/execution/durable progress or the secured 1M target.

The current core registry contains B0–B11 and SEC0/SEC1/SEC3: 15 gates.
T-M01–10, T-P01–10 and T-N09–12 remain registered, with R01–R12 unchanged in scope.
No core security, fee, execution, recovery or performance workload is withdrawn.
The two unused interop/bridge packages are removed from the core workspace.
Their historical source and tests remain recoverable at 038fe80.

Local EVM Shanghai standards, Solidity/client fixtures, account/storage proofs and
the pinned Ethereum execution corpus remain core correctness tests. They impose
no dependency on a remote chain, its RPC, consensus or confirmation speed.

## Verified evidence

- [B0](B0-20261001.md): historical 205-case gate, including then-authorized SEC0/
  INT0 contracts, zero ignored/failed, strict lint/format/release and structure.
- [B1](B1-20261001.md): 271 cases; real canonical state, journals, genesis,
  atomic recovery, snapshots, CLI, corruption and process-recovery acceptance.
- [Checkpoint repair](CI-20261001-checkpoint.md): 273 local cases, 792-file
  structure pass; hosted runs at b0b9efa/038fe80 pass 271/273 respectively.
- [Integrated B2 evidence](B2-20261001.md): full source-frozen 348-case gate passes,
  with format, strict lint, release, 1,061-file structure and 1,079-file ownership.
  The actual client enforces 18 flows, 12 Solidity sources, two independent public
  processes and one master composition; these are not added again to Cargo counts.
- B2's immutable reference corpus executes 3,495 Shanghai variants; EVE compares
  3,118 protected executions with its declared economic delta, rejects 84 valid
  Ethereum unprotected cases deliberately, and preserves 293 expected invalid
  cases. All 88 unprotected inputs are rejected. This is not root equivalence
  under Ethereum fee redistribution or a 120M live-chain profile.
- Redcat permission-only notices and automated ownership verification are
  implemented. Exact JSON/generated/upstream associations preserve fixture bytes
  and third-party rights. Ownership is mandatory in every implemented bulk gate.
- No existing registry identity/checksum was replaced. Current Cargo audit reports
  zero known vulnerabilities and two derivative/paste maintenance warnings; the
  32-package client audit reports zero vulnerabilities. Raw artifacts stay ignored.

## Core bulk ledger

| Bulk | Status | Next requirement |
|---|---|---|
| B0 | DONE | Historical verified foundations; retain all current core regressions |
| B1 | DONE | Verified complete local state/recovery; retain regressions in B2 |
| B2 | DONE | Verified serial RPC/client/resource/ownership/scope gate; retain regressions |
| B3 | PAUSED_BY_OWNER / INCOMPLETE | Complete gate failed at all-node crash recovery; draft checkpoint is not accepted integration |
| B4 | NOT_STARTED | Authenticated followers/recovery and isolated public persistence |
| B5 | NOT_STARTED | Native staking/work/rewards/slashing and supply invariants |
| B6 | NOT_STARTED | P2P, eligible endpoints and independent copied role distributions |
| B7 | NOT_STARTED | Serial/parallel equivalence and measured scheduling |
| B8 | NOT_STARTED | Master HA, migrations, releases and operations |
| B9 | NOT_STARTED | Regional functional devnet/testnet acceptance |
| B10 | NOT_STARTED | Sustained secured capacity and persistence measurements |
| B11 | NOT_STARTED | Verified 1M aggregate finalized target |

## Core security ledger

| Bulk | Status | Next requirement |
|---|---|---|
| SEC0 | DONE | Primitive/inventory boundary; no network PQ claim |
| SEC1 | NOT_STARTED | Real hybrid enforcement, majority tests and migration |
| SEC3 | NOT_STARTED | Core secure-profile recovery/release/capacity acceptance |

## Deferred separate programs

SEC2 and INT0–INT3 are DEFERRED_UNTIL_EVE_TESTNET, outside core dependencies.
INT0's old contract tests truly passed in B0; that historical result neither
authorizes current adapters nor completes an external integration. Plans 28/30
and config/deferred-programs retain future safety requirements and provenance.
No external route, custody, bridge runtime or prototype program is authorized now.

## Unachieved boundaries

DEVNET_ACCEPTED, CONSENSUS_RESILIENCE_TESTED, PQ_PROFILE_VERIFIED,
SECURITY_PROFILE_ACCEPTED and SCALE_TARGET_VERIFIED remain NOT_ACHIEVED.
The classical baseline assumes less than one-third Byzantine weighted power and
strictly more than two-thirds unique valid power. 51_PERCENT_CONTINUITY remains
UNSATISFIED_BY_BASELINE. Signer/native lifecycle has scoped passing development
evidence and awaits the complete B3 gate. Authenticated network recovery,
standalone packaging and sustained capacity remain unfinished.
No mainnet, real funds/custody, paid provisioning or visibility change is authorized.

Every DONE entry needs implementation and reviewed passing evidence. Ownership
notices, a clean index, root matching or local durability do not establish finality,
security-profile acceptance or 1M TPS. Update EVIDENCE/HANDOFF at each bulk/checkpoint.

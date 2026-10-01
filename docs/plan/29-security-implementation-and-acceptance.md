<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 29 — Security implementation bulks and acceptance

Status: mandatory core security queue, updated 2026-10-01 under D40. SEC0 has historical foundation evidence; complete runtime security acceptance is not achieved. SEC2 is `DEFERRED_UNTIL_EVE_TESTNET` for a separately authorized bridge program.

## SX01 — Execution contract

Run SEC0, SEC1 and SEC3 alongside B0–B11 from plan 23, not as optional post-mainnet cleanup. Read plans 26–27 before designing consensus signatures, key enrollment, recovery or authentication resource budgets. Preserve current core functional, ownership and structure gates. Plans 28/30 retain deferred bridge/adapter safety requirements; SEC2 and INT0–INT3 are outside current core selection and acceptance.

This track adds majority-adversary evaluation and post-quantum authorization throughout EVE. It does not authorize production genesis, spending or real signing/custody keys. EVE testnet is the earliest point at which the owner may scope later separate adapter programs; no automatic start is authorized. Missing deferred adapters or the two-EVE bridge fixture do not fail core security acceptance and are not represented as completed.

## SX02 — Dependency map

```text
B0 includes SEC0 inventory/spikes/test registration
B2+B3+B4+B5+B6 -> SEC1 authenticated secure profile
B8+B9+SEC1 -> SEC3 integrated core security acceptance
SEC3 -> secured-profile B10/B11 capacity acceptance

EVE testnet + subsequent owner scope -> separate SEC2/INT program planning
```

SEC0 is part of B0, not a separate circular dependency. Baseline B3 may first establish classical consensus as explicitly labeled development work. SEC1 must then integrate and test actual mandatory hybrid consensus authentication before any PQ claim. An ignored signature wrapper, lowered quorum or master-finality fallback cannot satisfy it.

Historical B0 included INT0 under its then-active scope. Its metadata tests and two-EVE reservations remain historical evidence, not current core prerequisites or a bridge acceptance result. Future SEC2/INT work consumes versioned public EVE proof/finality interfaces outside core; it cannot add remote-chain wait, client, custody or durability dependencies to EVE consensus/execution.

B9's DEVNET_ACCEPTED remains a functional checkpoint with its actual profile disclosed. It is not completion of core security requirements. B10/B11 classical throughput experiments are diagnostic only; secured-target evidence requires the active verified core profile. External-chain speed and route readiness do not constrain EVE's secured 1M acceptance.

## SEC0 — Inventory and executable integration decisions

Tasks:

- Map every core authorization path: votes/proposals, set changes, account/admin/recovery, staking, EVE followers/light clients, releases, transport and stored checkpoints.
- Map every relevant commitment/prehash and identity truncation. State the intended quantum security properties and unresolved risks; a signature parameter does not determine the whole chain's strength.
- Pin actual maintained crypto implementations and official algorithm vectors/errata. Smoke-test ML-DSA-65 sign/verify, serialization, size limits and cross-implementation verification without inventing APIs or certification.
- Inspect consensus-engine and EVE follower/light-client verification surfaces. Record a compile-tested path or concrete failure and a bounded replacement/extension plan. No fake after-the-fact PQ finality wrapper.
- Register core T-M01–T-M10 and T-P01–T-P10 with real expected outcomes and dependencies. Implement gate discovery so missing/zero current tests cannot pass.
- Retain the historical bridge/INT0 inventory and registrations with their actual source identity. Label SEC2/T-BR/INT/T-I `DEFERRED_UNTIL_EVE_TESTNET`; do not create new fixture, endpoint or adapter scaffolds in this goal.

Gate: versioned inventory, source/parameter pins, smoke-test outputs, attack assumptions, integration decision record and registered executable test entry points. A classical API-only baseline is explicitly marked incomplete for SEC1, not PQ-secure.

## SEC1 — Majority-adversary and secure authorization paths

Tasks: run supported and out-of-assumption consensus adversaries; implement actual hybrid enforcement in consensus messages/certificates/clients and secure-account authorization; bind enrolled keys and activation; secure release/recovery/control paths; test anti-downgrade, durable signing and stale anchors; resolve or clearly delimit commitment-strength claims.

Gate: T-M01–T-M10 and T-P01–T-P10 pass according to their stated outcomes. Simulated classical-key compromise cannot bypass mandatory enrolled PQ signatures. No master authority, classical-only recovery bypass or silent profile switch exists. Publish named fault assumptions and crypto coverage; unresolved required trust paths keep the secure profile incomplete.

Passing a test whose expected result is “majority withholding halts the baseline” demonstrates honest fault characterization, not 51% continuity. Keep any stronger majority-tolerance target `UNSATISFIED_BY_BASELINE` until a reviewed alternative with explicit assumptions satisfies it. Do not mark it DONE by editing the requirement.

## SEC2 — Deferred separate bridge program

State: `DEFERRED_UNTIL_EVE_TESTNET`. Subsequent owner scope is required before implementation, including the two-EVE fake-asset fixture. SEC2 is neither a failed current core gate nor completed bridge work. Its retained future requirements are route registry, authenticated source client, inclusion, atomic replay/backing accounting, bounded queues/limits, incident policy and recovery under [plan 28](28-bridge-security-and-finality.md).

Future program gate: T-BR01–T-BR12 pass for its named fixture/profile. Actual local consensus produces and authenticates the events. Malicious relayer/master input, stale/incorrect proofs, replay, reentrancy and wrong-network messages cannot move value. Bridge accounting and message consumption remain atomic through restart. These obligations do not add bridge state or remote verifiers to core.

A later EVE-to-EVE result is not proof of an external route. INT0–INT3 retain direction-specific future requirements under plan 30. Production route approval, audit, keys and funds remain owner gates; EVE PQ authorization cannot certify an external endpoint.

## SEC3 — Integrated security and capacity handoff

Tasks: fresh-checkout secure-profile deployment, independent verifier/cross-implementation checks, key-rotation/upgrade/incident drills, data-loss/master-outage recovery, gate/report integration, package independence and complete workload/resource accounting. Re-run core EVM, fee, serial/parallel and structure tests after security changes.

Gate: core security tests pass on the integrated revision; profile limitations are machine-readable and visible to operators; no undeclared classical control path remains in the protected core scope. Observable conflicting finality quarantines affected authentication/readiness, preserves evidence and cannot be cleared by unilateral master history selection. Missing independent audit or stronger fault-model proof is explicitly unresolved, not simulated away.

Run high-throughput measurements with real secure-profile checks enabled. Security-disabled or classical-only measurements cannot close the secured 1M target. No bridge run or external adapter is required for SEC3. Reports must disclose the deferred program scope rather than imply whole-route or custody security.

## SX03 — Test runner and evidence

The runner must select SEC0, SEC1 and SEC3 for current core security verification. Preserve SEC2 and INT0–INT3 IDs as explicitly deferred; requesting one must report `DEFERRED_UNTIL_EVE_TESTNET`, never a passing empty gate. Include ownership/structure checks and reject absent/stubbed current tests, unsupported requested coverage and zero selected cases. Historical commands/results retain their recorded revision; future program runners require their own scoped gate implementation.

Every report records commit/config/genesis, profile/parameter/key epochs, engine and crypto versions, actual test count, command and exit code, attack schedule, raw evidence hashes, timing and residual risks. Keep secret keys and real credentials out of logs. Test-only compromised keys must be visibly disposable local fixtures.

Track distinct statuses in `docs/execution/STATUS.md`:

```text
CONSENSUS_RESILIENCE_TESTED: NOT_ACHIEVED
PQ_PROFILE_VERIFIED: NOT_ACHIEVED
SECURITY_PROFILE_ACCEPTED: NOT_ACHIEVED
51_PERCENT_CONTINUITY: UNSATISFIED_BY_BASELINE
BRIDGE_DEVNET_ACCEPTED: DEFERRED_UNTIL_EVE_TESTNET
INTEROP_DEV_ACCEPTED: DEFERRED_UNTIL_EVE_TESTNET
EXTERNAL_BRIDGE_ROUTES: DISABLED_NOT_APPROVED
```

`SECURITY_PROFILE_ACCEPTED` requires SEC0, SEC1 and SEC3 implementation/evidence for the declared supported fault model and protected EVE core scope. This explicit D40 scope change does not weaken hybrid authentication or majority assumptions. It is not a mainnet audit, external route approval or immunity guarantee. A stronger unresolved user target remains separately open.

## SX04 — Agent ownership and continuation

Lead assigns requirement/test ownership and dependencies. Protocol/EVM owns consensus and account authorization; state/network/performance owns transport, availability and resource accounting; correctness/security owns core adversarial/crypto invariant tests; reviewer/integrator owns full-gate verification and evidence. Deferred program requirements are retained reference material, not an active agent assignment.

Follow plan 25 in new core crypto code: meaningful nested capability folders, one primary behavioral function per file, target 200 physical lines and reviewed 400/600 limits. Preserve package independence and current safety tests during refactoring. Future bridge code will require the same policy within its separate program.

Continue ready core/security work, commit coherent verified bulks, and update STATUS/EVIDENCE/HANDOFF. Do not start deferred adapters or stop at a security diagram or wrapper with unconditional verification success. Report genuine infrastructure/protocol/review blockers with evidence and the precise next action.

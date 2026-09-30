# 29 — Security implementation bulks and acceptance

Status: mandatory extension of the executable queue, added 2026-09-30. Specifications and tests described here are not implemented or passing yet.

## SX01 — Execution contract

Run this security track alongside B0–B11 from plan 23, not as an optional post-mainnet cleanup. Read plans 26–28 before designing consensus signatures, key enrollment, recovery, bridge proofs or their resource budgets. Preserve all earlier functional and structure gates.

This track adds majority-adversary evaluation, post-quantum authorization and a concrete local bridge fixture. It does not authorize launching a bridge, creating a production genesis, spending money or using real custody/signing keys. Missing external adapters do not block local tests or unrelated core development.

## SX02 — Dependency map

```text
B0 includes SEC0 inventory/spikes/test registration
B2+B3+B4+B5+B6 -> SEC1 authenticated secure profile
B3+B4+B5+B6 -> SEC2 classical bridge fixture development
SEC1 -> SEC2 hybrid end-to-end acceptance
B8+B9+SEC1+SEC2 -> SEC3 integrated security acceptance
SEC3 -> secured-profile B10/B11 capacity acceptance
```

SEC0 is part of B0, not a separate circular dependency. Baseline B3 may first establish classical consensus as explicitly labeled development work. SEC1 must then integrate and test actual mandatory hybrid consensus authentication before any PQ claim. Early classical bridge tests may run while SEC1 is being implemented; they do not satisfy the secure bridge gate.

B9's DEVNET_ACCEPTED remains a functional checkpoint with its actual profile disclosed. It is not completion of the owner's new security requirements. B10/B11 classical throughput experiments are diagnostic only; secured-target evidence requires the active verified profile.

## SEC0 — Inventory and executable integration decisions

Tasks:

- Map every authorization path: votes/proposals, set changes, account/admin/recovery, staking, source clients, bridge custody, releases, transport and stored checkpoints.
- Map every relevant commitment/prehash and identity truncation. State the intended quantum security properties and unresolved risks; a signature parameter does not determine the whole chain's strength.
- Pin actual maintained crypto implementations and official algorithm vectors/errata. Smoke-test ML-DSA-65 sign/verify, serialization, size limits and cross-implementation verification without inventing APIs or certification.
- Inspect consensus-engine and destination-verifier integration surfaces. Record a compile-tested path or concrete failure and a bounded replacement/extension plan. No fake after-the-fact PQ finality wrapper.
- Register T-M01–T-M10, T-P01–T-P10 and T-BR01–T-BR12 with real expected outcomes and dependencies. Implement gate discovery so missing/zero tests cannot pass.
- Fix local bridge fixture identities and fake-token configuration for two independent EVE devnets. External chain selection remains route-specific, not a guessed connector.

Gate: versioned inventory, source/parameter pins, smoke-test outputs, attack assumptions, integration decision record and registered executable test entry points. A classical API-only baseline is explicitly marked incomplete for SEC1, not PQ-secure.

## SEC1 — Majority-adversary and secure authorization paths

Tasks: run supported and out-of-assumption consensus adversaries; implement actual hybrid enforcement in consensus messages/certificates/clients and secure-account authorization; bind enrolled keys and activation; secure release/recovery/control paths; test anti-downgrade, durable signing and stale anchors; resolve or clearly delimit commitment-strength claims.

Gate: T-M01–T-M10 and T-P01–T-P10 pass according to their stated outcomes. Simulated classical-key compromise cannot bypass mandatory enrolled PQ signatures. No master authority, classical-only recovery bypass or silent profile switch exists. Publish named fault assumptions and crypto coverage; unresolved required trust paths keep the secure profile incomplete.

Passing a test whose expected result is “majority withholding halts the baseline” demonstrates honest fault characterization, not 51% continuity. Keep any stronger majority-tolerance target `UNSATISFIED_BY_BASELINE` until a reviewed alternative with explicit assumptions satisfies it. Do not mark it DONE by editing the requirement.

## SEC2 — Bridge implementation and recovery

Tasks: route registry, authenticated source client, receipt/event inclusion, replay ledger, exact lock/mint/burn/unlock accounting, fake-token two-network fixture, bounded queues, route limits, delay/pause/resume policy, key/profile upgrades and crash recovery. Use shared finality and crypto interfaces, not private master APIs.

Gate: T-BR01–T-BR12 pass for the named fixture/profile. Actual local consensus produces and authenticates the events. Malicious relayer/master input, stale/incorrect proofs, replay, reentrancy and wrong-network messages cannot move value. Bridge accounting and message consumption remain atomic through restart.

A local EVE-to-EVE result is not proof of an external route. Implementable adapter work may proceed when actual source data/specifications exist, but production route approval, audit and keys remain owner gates. Keep the exact missing prerequisite in HANDOFF rather than asking to postpone all work.

## SEC3 — Integrated security and capacity handoff

Tasks: fresh-checkout secure-profile deployment, independent verifier/cross-implementation checks, key-rotation/upgrade/incident drills, data-loss/master-outage recovery, gate/report integration, package independence and complete workload/resource accounting. Re-run core EVM, fee, serial/parallel and structure tests after security changes.

Gate: security tests pass on the integrated revision; profile/route limitations are machine-readable and visible to operators; no undeclared classical control path remains in the protected scope. Missing external audit or stronger fault-model proof is explicitly unresolved, not simulated away.

Run the bridge locally with fake funds only. Run high-throughput measurements with real secure-profile checks enabled; report bridge proof throughput separately from ordinary EVM TPS. Security-disabled or classical-only measurements cannot close the secured 1M target.

## SX03 — Test runner and evidence

Extend the B0 task runner to support `cargo xtask verify --security SEC0`, `SEC1`, `SEC2` and `SEC3`. These are required interfaces to implement, not available commands claimed by this document. Include structure checks and reject absent/stubbed tests, unsupported requested coverage and zero selected cases.

Every report records commit/config/genesis, profile/parameter/key epochs, engine and crypto versions, actual test count, command and exit code, attack schedule, raw evidence hashes, timing and residual risks. Keep secret keys and real credentials out of logs. Test-only compromised keys must be visibly disposable local fixtures.

Track distinct statuses in `docs/execution/STATUS.md`:

```text
CONSENSUS_RESILIENCE_TESTED: NOT_ACHIEVED
PQ_PROFILE_VERIFIED: NOT_ACHIEVED
BRIDGE_DEVNET_ACCEPTED: NOT_ACHIEVED
SECURITY_PROFILE_ACCEPTED: NOT_ACHIEVED
51_PERCENT_CONTINUITY: UNSATISFIED_BY_BASELINE
EXTERNAL_BRIDGE_ROUTES: DISABLED_NOT_APPROVED
```

`SECURITY_PROFILE_ACCEPTED` requires SEC0–SEC3 implementation/evidence for the declared supported fault model and protected crypto/bridge scope. It is not a mainnet audit, approval of every external chain, or an immunity guarantee. A stronger unresolved user target remains separately open.

## SX04 — Agent ownership and continuation

Lead assigns requirement/test ownership and dependencies. Protocol/EVM owns consensus and account authorization; state/network/performance owns transport, availability and resource accounting; correctness/security owns adversarial, crypto and bridge invariant tests; reviewer/integrator owns full-gate verification and evidence. Add specialized review tasks within these roles rather than making unverified crypto/security claims.

Follow plan 25 in all new crypto and bridge code: meaningful nested capability folders, one primary behavioral function per file, target 200 physical lines and reviewed 400/600 limits. Preserve package independence and existing safety tests during refactoring.

Continue ready work, commit coherent verified bulks, and update STATUS/EVIDENCE/HANDOFF. Do not stop at a security diagram or wrapper with unconditional verification success. Report genuine infrastructure/protocol/review blockers with evidence and the precise next action.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 26 — Consensus adversaries and majority-attack resilience

Status: mandatory security requirements, added 2026-09-30. Not an implemented defense, security proof, audit, or guarantee of majority-attack immunity.

## M01 — Requirement and scope

The owner requires resistance to majority attacks alongside post-quantum security in EVE core. Translate that objective into explicit adversary models, executable tests and residual-risk reports. Do not use “51% resistant” as an unqualified acceptance label. EVM execution, consensus and cryptographic authentication are core boundaries. Bridge custody belongs to a future separate program, `DEFERRED_UNTIL_EVE_TESTNET` under D40.

This plan extends plans 12, 17, 19 and 20. Plan 27 covers mandatory core quantum authentication; plan 28 retains deferred bridge safety requirements. Preserve validator-owned finality, protected follower-only masters, the 40/30/30 fee split and plan 25's recursive one-function-file policy.

## M02 — Baseline fault assumptions

The initial CometBFT profile uses weighted BFT with a strict quorum: for total active power T and valid unique signer power S, accept only when `3*S > 2*T`. Use checked arithmetic, the correct historical validator set, and the engine's locking/signing rules. This is not a count of public nodes, stakers, IP addresses or countries.

The baseline safety guarantee assumes less than one-third Byzantine voting power. Liveness also depends on sufficient participating honest power and eventual network delivery. A coalition of one-third or more can withhold votes and prevent progress. With 51% Byzantine power the baseline is outside its safety assumptions: it cannot promise either continuous operation or protection against all conflicting-finality scenarios.

A 51% coalition cannot alone satisfy a greater-than-two-thirds certificate, but that fact is NOT proof of safety: Byzantine equivocation and different honest views must be considered. Two conflicting quorums can overlap in malicious signers. Never turn “cannot sign a quorum alone” into “cannot create a fork.”

These limits follow the upstream [CometBFT consensus specification](https://github.com/cometbft/cometbft/blob/main/spec/consensus/consensus.md), especially its safety, accountability and censorship sections. Pin the actual engine revision during implementation; do not replace a proof with this prose.

## M03 — Separate guarantees

| Property | Required interpretation |
|---|---|
| Execution validity | Honest replaying nodes reject invalid transitions, including forged spends, regardless of a master's assertion. |
| Agreement/finality | Honest nodes agree under the declared consensus fault model; a valid-looking certificate beyond that model is not a universal guarantee. |
| Availability | Report progress, censorship and halt conditions separately; preserving data is not producing new finality. |
| Accountability | Preserve verifiable evidence where available; slashing is a deterrent and response, not prevention or automatic recovery of bridge losses. |
| Recovery | Never silently replace finalized history, reset signer safety or let master/admin keys choose a fork. |

An attacker controlling enough legitimate signing power is not solved by stronger signature algorithms. Post-quantum signatures prevent a different class of key-forgery attacks; they do not stop an authorized malicious quorum from signing.

## M04 — Required defenses within the supported model

Validate proposals through deterministic execution, full necessary data availability and authenticated parent state. Keep durable anti-equivocation records, one active signing owner per identity, and signer fencing during recovery. Never reduce quorum automatically when validators go offline.

Use bonded stake and deterministic activation/removal delays. Keep unbonding, evidence retention and checkpoint trust periods consistent across consensus and EVE clients. Future adapters must honor those EVE periods through public interfaces. Validator changes must be authenticated by previous valid history, not a list from a master or relayer.

Measure stake concentration, delegation concentration and correlated operator/cloud/region exposure. Report the available evidence and its limitations. One operator can create many keys: per-key caps, country IDs, node signatures and IP diversity do not establish independent ownership or Sybil resistance. Do not introduce identity-based slashing based on unverifiable operator guesses.

Keep RPC, synchronization and consensus queues separate and bounded. Support diverse authenticated bootstrap sources, peer rotation and eclipse/partition tests. Multiple RPC providers reading the same chain are transport redundancy, not independent consensus security.

A known finalized checkpoint cannot be replaced by a conflicting peer assertion. Preserve conflicting certificates/evidence, quarantine the affected authenticated-state/readiness path and expose a security alarm once a verifiable conflict is observed. A unilateral master action cannot replace history or clear the conflict. Detection is not guaranteed to precede damage, especially under censorship or partition; do not make an impossible global “all nodes instantly pause” promise. Future bridge programs must apply their own route containment under plan 28.

## M05 — Behavior outside the fault assumptions

For tests with one-third or more Byzantine power, record which properties fail or remain true for the tested schedule. An observed safe run does not establish safety for every adversarial schedule. Include withholding, equivocation, censorship, stale-set use, partitions, adaptive corruption and long-range recovery cases.

No majority-attack recovery may grant production finality to master, release, emergency or developer keys. An exceptional social recovery is an explicit owner/operator-coordinated network decision with visible fork/checkpoint identity and documented consequences, not an automatic consensus fallback.

If continuous correct finality under 51% Byzantine active power remains a strict product requirement, mark it `UNSATISFIED_BY_BASELINE`. Research alternative fault/network/trust models with stated assumptions, protocol reasoning and adversarial evidence. Do not silently change the requirement or claim a protocol-name change proves it. Additional independent trust or stronger synchrony assumptions must be disclosed and approved before use.

## M06 — Deferred bridge containment reference

This subsection applies only to a subsequently authorized separate bridge program after EVE testnet. It adds no current core bridge gate or remote-chain dependency.

A bridge verifies the source chain's finalized history under a named trust model. If that trust model fails, valid-looking source proofs may no longer reflect a unique economic history. Limits, delays, independent monitoring and pause mechanisms reduce exposure but do not repair compromised source consensus.

Keep per-asset/per-route limits, pending-transfer queues and authenticated pause evidence in plan 28. A pause authority can only pause; it cannot invent a deposit, redirect a recipient, choose chain truth or unlock arbitrary balances. No emergency bypass exists around proof verification.

## M07 — Acceptance tests

Run attacks only in task-owned local/devnet resources with fake tokens. Tests must use actual consensus/verification paths, not a mock boolean called `is_safe`.

| Test | Required result/evidence |
|---|---|
| T-M01 | Weighted boundary vectors: just below one-third faulty power, one-third, 51%, exactly two-thirds signer power, and strictly greater than two-thirds. Integer rounding and duplicate identities are covered. |
| T-M02 | Under the supported fault model, equivocation/partition schedules do not produce two finalized histories at honest nodes; normal progress resumes under the declared liveness conditions. |
| T-M03 | At least one-third withholding demonstrates stalled finality without lowered quorum or master takeover. |
| T-M04 | A 51% attacker-only certificate is rejected. Separately, synthetic conflicting-quorum and adversarial-engine schedules demonstrate why this does not establish 51% safety. Preserve attack transcripts and classify assumption violations honestly. |
| T-M05 | A compromised master or RPC source cannot change authenticated state, validator membership or accepted historical checkpoints. |
| T-M06 | Key cloning, signer restart, stale backups and corrupted signing state fail safely without equivocation by honest signers. |
| T-M07 | Public-node multiplication does not increase voting power; stake/set changes and delegation concentration are auditable. |
| T-M08 | Long-range/stale-checkpoint, eclipse and delayed-evidence cases produce explicit EVE recovery/client restrictions rather than fresh trust in the newest supplied signature. |
| T-M09 | On observable conflicting finality, affected EVE authentication/readiness is quarantined, evidence is retained and a unilateral master cannot replace history or clear the conflict. Future route containment is a deferred program extension. |
| T-M10 | Honest full replay rejects invalid state despite attacker-generated metadata; report separately what a non-replaying light client can establish. |

## M08 — Evidence and completion

Publish the tested engine/configuration, validator weights, adversary powers, network schedule/seed, checkpoints, observed roots, vote transcripts, halt/recovery behavior and exact failure assumptions. Distinguish test evidence from formal reasoning and independent review.

`CONSENSUS_RESILIENCE_TESTED` is scoped to the published fault model. It is not “51% immune.” Keep any stronger unmet requirement visible in STATUS and HANDOFF. A passing laboratory suite does not certify mainnet economic security.

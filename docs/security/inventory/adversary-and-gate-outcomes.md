<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Adversary assumptions and registered expected outcomes

The CometBFT development baseline accepts unique weighted signer power S only
when `3*S > 2*T`. Safety assumes Byzantine voting power below one third;
liveness additionally needs adequate honest participation and eventual delivery.
At one third withholding can halt finality. A 51% coalition cannot alone form a
strict two-thirds certificate, but conflicting quorums can overlap in malicious
signers. That observation is not a 51% safety proof. Required correct continuous
finality at 51% remains UNSATISFIED_BY_BASELINE.

These are expected outcomes for registration and later real paths, not passing
attack simulations. Full gate discovery must fail when an implementation/test is
missing, empty, stubbed or skipped. B0 primitive/metadata tests cover only their
explicit subset. D40 preserves core T-M01–T-M10/T-P01–T-P10 while deferring
SEC2/T-BR/INT/T-I to separate programs, `DEFERRED_UNTIL_EVE_TESTNET`, subject to
subsequent owner scope. Deferral is neither a pass nor a failed core gate.

| Gate | Expected outcome and required implementation |
|---|---|
| T-M01 | Checked weighted boundaries, exact two-thirds rejection, duplicates rejected; below one-third/one-third/51% modeled distinctly |
| T-M02 | Real engine schedules preserve honest agreement below supported fault threshold and recover eventual progress |
| T-M03 | At least one-third withholding stalls without quorum reduction or master takeover |
| T-M04 | Reject 51% attacker-only certificate; separate conflicting-quorum/engine schedules show violated assumptions without immunity claims |
| T-M05 | Compromised master/RPC cannot change authenticated state/set/checkpoint |
| T-M06 | Clones, restart, stale backups and corrupt durable signing state fail safely |
| T-M07 | Public-node count grants zero voting power; actual set/stake/delegation evidence is auditable |
| T-M08 | Stale/long-range anchors, eclipse and delayed evidence restrict EVE recovery/client trust rather than creating new trust |
| T-M09 | Observable conflicting finality quarantines affected EVE authentication/readiness and preserves evidence; master cannot choose history or clear conflict |
| T-M10 | Honest full replay rejects invalid execution metadata; light-client limitations reported separately |
| T-P01 | Standard vectors and maintained two-way implementation interoperability; malformed encoding and contexts rejected |
| T-P02 | Where hybrid is required, reject classical-only/PQ-only, mismatched keys/messages, wrong domains and duplicate signers |
| T-P03 | Actual vote/commit/client/protected-account paths still require enrolled PQ authorization after simulated classical compromise |
| T-P04 | Enrollment, activation, rotation, retirement, stale anchors and mixed versions preserve history and prohibit downgrade |
| T-P05 | Protected admin/recovery/permit/session and staking/control paths prohibit classical fallback and enforce nonce/replay/expiry |
| T-P06 | Crash/retry/backup/restore, oversized malformed signatures and exhaustion preserve signing safety and bounded queues |
| T-P07 | Full RPC/execution/finality/master/sync path enforces the profile and authenticated H–H+1 commitment binding |
| T-P08 | Real active-profile byte/CPU/storage benchmarks preserve serial/parallel semantics |
| T-P09 | Release/upgrade/recovery/transport inventory exposes every classical bypass and fails on required missing coverage |
| T-P10 | Commitment/address/proof analysis states residual assumptions rather than inferring whole-chain security from signatures |

## Deferred bridge-program gate outcomes

T-BR01–T-BR12 below are retained future requirements outside core gate selection.
Future bridge account/conflict handling extends core T-P05/T-M09 without replacing
their EVE coverage. No bridge fixture, route or remote-chain client is authorized
before EVE testnet and subsequent owner scope.

| Deferred gate | Required future program outcome |
|---|---|
| T-BR01 | Two real local EVE networks transfer fake assets with real source consensus/inclusion and custody |
| T-BR02 | Wrong chain/genesis/route/custody/recipient/amount/profile/proof cannot move value |
| T-BR03 | Replay, duplicate delivery and restart preserve exactly-once atomic effects |
| T-BR04 | Revert/lookalike/invalid inclusion/H–H+1 misbinding rejected |
| T-BR05 | Halt/stale trust/malicious source/missing data never bypass proofs or create emergency mint |
| T-BR06 | Exposure caps, bounded delay queues, authenticated pause and authorized reconciliation work under faults |
| T-BR07 | Pending deposits/burns/wrapped liabilities and exact decimals conserve backing; unsupported tokens fail |
| T-BR08 | Reentrancy, admin/upgrade takeover and retired keys rejected under active profile |
| T-BR09 | Actual destination path checks both signatures; external classical assumptions remain visible |
| T-BR10 | Observed conflicting finality stops affected route; report delayed-detection exposure |
| T-BR11 | No timeout-only refund races a delayed mint; unsupported refund remains disabled |
| T-BR12 | Real proof/PQ gas/bytes/latency/invalid-input verification costs are bounded and measured |

Threats requiring actual engine/runtime evidence include withholding, equivocation,
censorship, partitions, adaptive corruption, stale-set/long-range history, cloned
signers, missing data and source-finality conflicts. Laboratory observations above
the baseline threshold remain schedule-specific outcomes, never a new safety proof.
If a later bridge program is authorized, caps/watchers reduce exposure under
assumptions; they do not repair source consensus or guarantee detection before damage.

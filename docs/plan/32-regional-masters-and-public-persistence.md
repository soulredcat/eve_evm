<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 32 — Regional masters and RAM-first public persistence

Status: owner topology and persistence clarification recorded 2026-09-30. Documentation only; no runtime, latency isolation, crash recovery, failover, or capacity result is claimed.

## RP01 — Accepted roles and rollout

Start the separated development topology with one master follower, one public node, and the existing four-validator consensus baseline. A single-validator test producer remains an explicitly local development harness. Multiple logical development processes on one host do not demonstrate independent failure domains.

One master may serve several public nodes through bounded regional data-serving endpoints/relays. The intended evolution is two masters exchanging verified finalized history, followed by a planned ten-master regional deployment. These are topology intentions, not permission to purchase infrastructure, launch mainnet, choose production keys, or bypass the existing release gates.

Public nodes receive transactions, relay them through P2P, and serve verified state. Validators execute/validate proposals before voting and decide finality. Masters verify and persist finalized history, snapshots, and recovery data on durable storage such as NVMe. Ordinary transactions and consensus do not wait for a master acknowledgement.

Adding public nodes increases ingress/query capacity and distribution options. Adding full validators or full master replicas repeats validation/storage work; it does not multiply unique finalized TPS. The 1M aggregate finalized TPS target and required workload/security profile remain unchanged.

## RP02 — Zones and internal master topology

Introduce `zone_id` as typed operational metadata for routing, placement, and storage namespaces from the initial implementation. It is distinct from node identity, immutable genesis/network identity, EVM chain ID, and validator voting power. Moving a node or choosing another source cannot change the chain it verifies.

Public nodes need not enumerate private master hosts or know their database, management, or replication topology. They use a preferred logical regional sync endpoint and authenticated network/profile/proof data. A bounded serving API or relay exposes finalized objects, not database handles, filesystem mounts, admin access, or installation authority. Private transport credentials are not distributed to permissionless public nodes.

An endpoint can hide backend operational details; this is not a guarantee that physical hosts can never be inferred. Authentication and consensus verification remain mandatory. Zone IDs do not authorize independent writes, shard ownership, conflicting roots, or asynchronous cross-zone EVM calls. Actual partitioning remains gated by plan 21.

## RP03 — Preferred source selection and failover

Public may use one preferred sync endpoint at a time while retaining diverse P2P routes and fallback discovery. An endpoint outage must not make the master a mandatory transaction gateway. Existing public nodes can continue from validator/peer data when masters are offline and quorum, required data, and retention capacity remain available.

Source eligibility precedes speed ranking: verify configured endpoint/transport identity, immutable network identity, supported protocol/security profile, data provenance, and relevant validator/commitment history. A master or endpoint identity never authenticates arbitrary state.

Verify historical records against the profile/key epoch applicable at their height. Valid older authenticated history may serve replay or historical recovery while its source is ineligible as a fresh head; lag is not proof of forgery. Reject invalid proofs, wrong networks, and height/profile mismatches without treating every earlier profile as a downgrade attempt.

Choose a nearby eligible source using measured service response latency, useful throughput, error rate, load, and lag against independently authenticated history. ICMP ping alone is insufficient. A claimed remote height is untrusted until verified; one source alone cannot establish that its valid older history is the freshest available history. Report unknown freshness honestly when corroborating data is unavailable.

Bound discovery/probing, connections, retries, and downloads. Use deadlines, backoff, and hysteresis/hold-down rules to avoid rapid source switching. B0 freezes versioned source-selection and readiness limits; local latency scores affect routing only, never consensus arithmetic or reward scoring.

Fail over to another eligible endpoint or peer after timeout, invalid data, unsupported profile, or excessive verified lag. Stage downloads, authenticate anchors, and verify ordered delta bases/roots before activation. Never lower verification requirements or accept a stale untrusted checkpoint because it responds faster.

## RP04 — Public working state and durable recovery

The default `PUBLIC` profile keeps active verified state and bounded query/execution caches in RAM while retaining finalized block payloads, necessary receipts/metadata, authenticated validator/profile history, and recoverable state checkpoints on durable storage. A RAM-only non-voting replica remains an explicitly ephemeral development alternative; it does not satisfy default public persistence acceptance.

Current-state RPC captures one verified RAM view. Historical reads outside RAM use separately budgeted storage requests and return explicit unavailable/pruned status when needed. RAM state, mempool, overlays, storage buffers, and database/OS caches all consume the operator's finite resource budget; "full memory" does not mean unbounded allocation.

Public recovery storage may retain a consistent checkpoint plus the complete verified block sequence needed to replay through its durable recovery height instead of rewriting the whole hot state on every block. Block hashes or headers alone are insufficient. Persist all execution/authentication inputs needed for exact replay, including version/config history, and verify restored EVM/system roots and receipts.

Mempool admission is distinct from finality and durable history. Pending transactions need an explicit persistence or resubmission policy; a returned transaction hash is not a promise of finalized or crash-safe storage.

## RP05 — Isolated storage path

1. Verify finalized input and its applicable commitment/execution provenance; preserve plan 12's execution-before-vote requirement and H/H+1 authentication rules.
2. Reserve bounded handoff capacity and apply the ordered transition to an immutable/versioned RAM view. Publish a complete applied view under the declared readiness policy, never a partially mutated state.
3. Hand an ordered immutable recovery batch to a dedicated storage worker with one commit owner per namespace. The worker cannot read mutable speculative state or hold a global state lock across disk/network waits.
4. Write payloads and atomically publish references/metadata with the required WAL/sync policy from plan 14. Advance the durable recovery marker only when the checkpoint-to-height sequence is complete and actually durable.
5. Produce consistent checkpoints and snapshots under separate budgets. Publish manifests only after referenced content is complete and its provenance verified.

Use reviewed database WAL/write-batch facilities and an explicitly ordered block-segment protocol where needed; do not invent another general-purpose WAL. Asynchronous public persistence does not relax atomic full-state-store commits or validator signing durability.

Expose separately typed finalized-input, execution-applied, durable-recovery, checkpoint, authenticated-state, and oldest-retained heights, plus queue bytes/count/oldest age and persistence errors. A queued batch or OS page-cache write is not durable. Post-state authentication may require the next certified header; do not substitute finalized block height for authenticated post-state height.

Public may serve verified RAM state ahead of its local durable marker only within predeclared lag/readiness limits and recoverability policy. Storage failure cannot silently become "saved". Non-critical archival/index work must not starve execution or consensus when roles share a host.

## RP06 — Saturation, crashes, and retention

Bound queue bytes, batch count/size, queue age, applied-to-durable lag, cache/overlay allocation, worker CPU, concurrent reads, compaction, snapshot, and catch-up work. Reserve execution/consensus capacity where roles share resources. Thread/process separation reduces coupling but does not eliminate shared CPU, memory-bandwidth, or I/O contention; zero overhead is not an acceptance claim.

If limits or disk capacity are threatened, expose degraded status and apply bounded backpressure, shed non-critical work, or stop advancing local RAM application/readiness according to policy. Never grow an unbounded queue, discard the only recoverable finalized data, disable sync safety, or report a stale view as current. A lagging public node does not change validator finality; network continuation still requires quorum and available durable data.

After a public crash or power loss, stop readiness, recover the last complete local durable checkpoint/block sequence, replay exactly once, and retrieve any missing queued tail from authenticated durable peers even when all masters are offline. If the tail is unavailable, report the exact unavailable range and keep readiness false (`NOT_READY`) instead of fabricating balances or durability. Explicitly labelled historical reads may remain available under policy; they do not imply current-head readiness.

Validators retain durable recent finalized data and recoverable state independently of masters. Their anti-double-sign records must be durable before signatures/votes are released. Never prune the last recovery copy; preserve plan 15's retention and independently operated source requirements. A checkpoint without the needed replay tail cannot justify pruning that tail.

## RP07 — Master replication

Each master has its own durable namespace and independently verifies imported finalized history, whether supplied by validators, relays, or another master. Two-way synchronization exchanges authenticated blocks, deltas, and snapshots, not concurrent arbitrary database-row writes. Shared physical namespaces require single-writer fencing; signer fencing remains a separate validator obligation.

Masters may lag or partition. Rejoin from the last complete durable/authenticated anchor, verify missing history and profile/set transitions, and converge at the same height to identical commitments. Conflicting/invalid inputs are rejected; no master vote, source majority, or "newest master" selects canonical history.

Every full master replica still ingests its configured full finalized stream and needs measured catch-up headroom. Regional serving, chunk caching, and peer fanout can distribute downloads without requiring a privileged direct master connection for each public node. Ten replicas are not evidence of tenfold execution capacity.

## RP08 — Evidence, tests, and bulk ownership

B0 freezes compile-tested types, source-selection/readiness/storage budgets, watermark semantics, and gate registration through bounded interface/storage spikes. It does not claim later distributed runtime tests passed. B1 implements durable commit/recovery contracts; B4 integrates public persistence, master-offline replay, and retention; B6 implements regional discovery/serving; B8 verifies independent master replication/failover. B9 reruns the integrated gates; B10 measures interference and sustained capacity with real persistence/security enabled.

| Test | Required outcome | Owning bulk |
|---|---|---|
| T-N09 | Slow/failing disk and simultaneous snapshot/index work cannot hold RAM state locks; queue/memory/lag limits and declared readiness/backpressure behavior remain enforced, with measured RPC/execution interference | B4 |
| T-N10 | Public process/power-loss recovery with masters offline restores a complete durable prefix, retrieves the unsynced tail from authenticated durable peers, and reproduces roots/receipts without duplicate effects; an unavailable tail is explicit and readiness remains false | B4 |
| T-N11 | Prefer a nearby eligible source; reject invalid/wrong-network/height-profile-mismatched data, distinguish valid older replay history from a fresh head, handle uncertain freshness, fail over with bounded discovery/switching, and retain master-independent P2P ingress | B6 |
| T-N12 | Two independent master stores survive outage/partition and catch up to identical roots at the same finalized height; invalid history and any master-finality takeover are rejected | B8 |

Retain every core T-S/T-N/T-G, structure, ownership, security, fee and serial/parallel test. D40 defers external interoperability until testnet rather than removing any core recovery case. Test declared limits and loss of relevant peers without claiming impossible recovery; label the four-validator same-host/emulated faults honestly.

Reports separate finalized/applied/durable/authenticated rates and latency, RAM footprint, queue slope, disk bytes/write amplification, snapshot/catch-up load, retention headroom, and per-master ingest. Define workload, security profile, resource limits, and p99/readiness objectives before measurement. A fast RAM-only run cannot satisfy durable public acceptance or the secured 1M target.

Required shared test source and sanitized fixtures stay versioned. Experiments and raw reports stay in ignored `local-tests/`/`artifacts/`; English compact evidence belongs in `docs/execution/`. This document registers implementation obligations, not passing tests, available deployment resources, or MAINNET_READY.

# 15 — Network, synchronization and data availability

## N01 — Topology and identities

The consensus engine provides validator block/transaction propagation in the first devnet. Implement public request/response and bulk distribution behind a transport interface; avoid two incompatible mempools or duplicate gossip of the same transaction. Public nodes retain multiple peers/relays and may prefer one eligible logical endpoint for bulk synchronization. No mandatory master hub or global public-node-count limit is introduced.

Follow [plan 32](32-regional-masters-and-public-persistence.md): logical regional gateways may hide internal master inventories while serving many public nodes. A public node needs endpoint/network identity and verifiable data, not master database credentials or knowledge of which internal master served it. Zone IDs select routes; they do not grant consensus authority. Private routing does not establish data correctness or guarantee host anonymity.

Authenticate network/profile and proof provenance first; rank healthy eligible sources by independently verified freshness, then measured service RTT and sustained throughput. Advertised heights and the smallest ICMP ping are insufficient; report unknown freshness when no corroborating authenticated history is available. Use bounded probes, hysteresis and cooldowns; retain fallback discovery/peers when the preferred endpoint is stale, invalid or unavailable. Transaction propagation and validator consensus remain independent of this preference.

Handshake fields: network name, immutable genesis hash, EVM chain ID, protocol range, wire version, peer identity, capabilities and finalized/applied heights. Reject mismatched networks and unsupported protocol versions before expensive sync. A peer key proves endpoint identity, not validator membership or master authority. Master internal access additionally uses operator-controlled mTLS/allowlists; permissionless public peers do not receive privileged certificates.

Use peer diversity, per-peer quotas, global resource ceilings, exponential retry with jitter, content-based duplicate suppression and bounded queues. A million Sybil peers cannot create a million privileged master connections.

## N02 — Logical APIs

Implement versioned messages for:

- `Status` and peer capability negotiation.
- `SubmitTransaction`/transaction propagation with standard tx identity.
- `GetHeaders`, `GetFinalityProof`, `GetValidatorTransitions`.
- `GetBlockRange`, `GetReceipts`, and content-addressed block retrieval.
- `GetSnapshotManifest`, `GetSnapshotChunk`.
- `GetDeltaRange`, `GetDeltaChunk`, and resumable subscriptions.

Administrative methods and software installation are absent from this protocol. Define stable errors: WRONG_NETWORK, UNSUPPORTED_VERSION, NOT_READY, PRUNED, GAP, BAD_PROOF, RESOURCE_LIMIT and RETRY_LATER. Retries are idempotent.

Transport message envelope includes version, type, request ID, bounded payload length and payload checksum where needed. Bulk and consensus have separate scheduling/bandwidth budgets even on a shared NIC. Pin actual codec schemas and limits in B0.

## N03 — Bootstrap trust

Start from a locally configured genesis or a separately trusted recent checkpoint, not whatever root the first seed returns. Verify consensus headers, active validator-set changes, signatures and commitment binding. Implement the selected light client's trust-period rules; an expired checkpoint is not refreshed by accepting an unverified master signature.

For development, a 24-hour checkpoint trust period and seven-day unbonding minimum are defaults; both must be validated against evidence windows and the chosen engine. Fresh genesis replay is a safe verification path when practical. Multiple serving peers improve availability but do not replace a trust anchor. [Primary light-client specification](https://docs.cosmos.network/cometbft/latest/spec/light-client/verification).

## N04 — Snapshot format

A manifest includes format version, genesis/config digest, execution height/hash, both state roots, application commitment, authenticated consensus proof reference, chunk count, canonical uncompressed length, chunk hashes, compression algorithm, schema version and retention information. Chunk identity hashes canonical uncompressed bytes. Optional source signatures authenticate delivery but cannot authorize state.

Initial chunk target: 4 MiB uncompressed; maximum decompressed chunk: 8 MiB. Bound total declared snapshot bytes by operator capacity before allocation, concurrent chunks by configuration and per-peer in-flight bytes. Reject unsafe compression ratios, invalid paths, duplicate keys, unexpected ordering and schema mismatches.

Download into a staging namespace, retry missing chunks from other peers, verify chunks, reconstruct both state roots and application commitment, verify consensus anchoring, then atomically activate. Never partially replace the live database. Retain the old valid snapshot until the new one is usable. Exports must use a consistent finalized state view.

## N05 — Delta protocol

A logical delta range records base/target heights, roots, execution hashes, ordered block references, canonical key operations for both state domains, deletion markers, chunks and proof references. The base must match the receiver's actual state; never apply a later delta on an unknown parent.

Apply consecutive deltas to an isolated view. Verify complete content, commitment provenance and computed destination roots before activation. Duplicated delivery is harmless; missing/reordered deltas trigger bounded catch-up. Resume cursors bind network, snapshot/base hash and last verified sequence, not just an integer height.

Compression reduces some bytes, not the need to provide account/storage changes and transaction data. Batch roots by themselves cannot reconstruct state or prove execution. Fast import verifies authenticated outcomes; independent replay remains a separate verification mode.

## N06 — Durability independent of master

Every validator maintains durable recent finalized blocks plus a recoverable state checkpoint/log, alongside durable signing safety. The initial retention minimum is 10,000 finalized blocks or 24 hours, whichever preserves more history, and must also cover the required evidence and recovery windows. Development test profiles may shorten these explicitly, never production silently.

Before pruning the last local recovery path, require verified recoverability from at least two independently operated durable sources or keep the data locally. A remote receipt is an availability assertion, not a mathematical guarantee of permanent storage; rehearse retrieval and record operator/failure-domain assumptions. The four-validator devnet retains all data until pruning tests exist.

Master offline behavior: retain and replicate finalized data among validators/peers; track backlog and disk capacity. No ordinary per-block master ack is required. If retention capacity is threatened, stop unsafe pruning, shed admission or safely stop proposing/signing according to the resource policy; never discard the only finalized data to maintain a TPS graph.

Default public nodes keep verified working state in RAM and durable finalized blocks plus recoverable checkpoints locally. An explicit ephemeral non-voting development profile may be rebuilt, but cannot claim local durability. An ephemeral signing validator is forbidden. Restore of all RAM replicas must not depend on a single stale master copy that lacks recently finalized blocks.

One, then two, then a planned ten masters are independent verified replicas of the same finalized chain. Each verifies imported data and keeps its own durable store. They may synchronize through peers or other masters without choosing finality, merging conflicting balances, or electing a master consensus leader. Full replication repeats the committed data workload; additional masters improve distribution/redundancy, not transaction capacity by simple multiplication.

## N07 — Admission limits

Initial local profile: 64 active peer connections per process, 8 concurrent bulk requests, 32 MiB in-flight bulk budget per peer, 1 MiB ordinary request envelope excluding declared chunk streams. These are local resource defaults, not consensus or public-node-count limits. Operator limits may tighten them without changing valid block semantics.

Keep dedicated quotas for `eth_call`, log scans, subscriptions, chunk requests and invalid signatures. Rate limiting by peer/IP is not claimed to solve Sybil resistance. Validate small headers before large allocation. On peer abuse, retain useful network diversity rather than disconnecting every independent route.

## N08 — Public persistence isolation and recovery

Hand verified immutable finalized batches to an isolated ordered storage worker. Batches bind network/profile, block sequence/hash, proofs, replay data and checkpoint references. Capture checkpoint state at one verified height; the worker must not reconstruct it from changing live RAM. Persist payloads, references and actual durable markers under plan 14's ordered crash-safe protocol. An enqueue, write completion without required sync, or page-cache update is not durability.

Do not hold a global RAM-state/execution lock across database writes, fsync, compaction or snapshot export. Bound queue bytes/items/age, persistence lag, worker CPU/IO, checkpoint/snapshot memory and disk headroom in a predeclared profile. Shared CPU, memory and devices still have costs; isolation is not a zero-overhead guarantee. B0 pins the measurement contract and bounded storage spike; B4/B6 supply runtime evidence and B10 measures sustained interference.

Readiness and overload behavior follow the predeclared applied/durable-lag policy. At a resource or lag limit, apply bounded backpressure, stop new sync work or shed service according to that policy; report NOT_READY/RESOURCE_LIMIT rather than silently growing RAM or dropping the last recoverable copy. Keep consensus-finalized, verified-applied, durable, authenticated-state and authenticated-checkpoint watermarks distinct, preserving plans 12/14's H/H+1 post-state binding. RPC views remain internally consistent and never advertise RAM progress as durable storage.

Stopping every master while public/validator peers remain active must preserve verified public operation within these budgets and continued durable public recovery data. After public process/power loss, discard queued unsynced RAM, recover only complete local durable batches/checkpoints, then authenticate and replay any missing finalized tail from retained durable peers with masters still offline. Reconcile roots/receipts and restore readiness only when the configured recovery/lag policy passes. If no valid retained copy exists, remain not ready and report the exact gap; do not invent balances, history or a power-loss guarantee.

## Acceptance

T-N01: public node bootstraps from peers with master unavailable; every root matches.
T-N02: wrong-network, stale checkpoint, wrong validator set and wrong-height anchors fail.
T-N03: corrupted/missing/reordered/repeated chunks and deltas recover or reject deterministically.
T-N04: interrupted snapshot never destroys the live state; resume works after restart.
T-N05: master offline while blocks continue, then catches up byte-for-byte without voting.
T-N06: floods, decompression bombs, oversized manifests and slow peers remain within resource budgets.
T-N07: pruning refuses to destroy the last recoverable finalized history.
T-N08: adding public peers does not require one direct master stream per node.
T-N09: public disk/fsync/compaction stalls leave no global RAM lock held across IO; queues, memory, worker resources and lag remain within predeclared budgets, with correct overload/readiness transitions and truthful durable markers.
T-N10: stop all masters, continue finalized public progress, then crash/power-loss the public node with an unsynced tail; recover complete local data plus authenticated durable peer tail exactly once, match oracle roots/receipts, and fail not-ready when every valid tail copy is unavailable.
T-N11: nearest-endpoint selection rejects wrong-network data, invalid proofs, fabricated height claims and profiles inconsistent with the applicable height/epoch; valid older authenticated history remains usable for replay but does not prove a fresh head. Unknown freshness is explicit, health/freshness precede measured service RTT/throughput, and bounded probes/hysteresis/fallback peers need no internal master inventory.
T-N12: two independent masters partition, lose/recover one source and catch up to the same authenticated finalized height/root/history without voting or overriding validator finality; public failover remains verified and no shared-storage split writer appears.

T-N09/T-N10 are B4 recovery gates, T-N11 is a B6 network gate, and T-N12 is a B8 HA gate. B0 registers them with concrete fixtures/dependencies; B9 reruns them on the integrated revision. These are required tests, currently NOT_RUN, not evidence from this specification.

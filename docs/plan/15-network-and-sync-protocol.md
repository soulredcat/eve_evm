# 15 — Network, synchronization and data availability

## N01 — Topology and identities

The consensus engine provides validator block/transaction propagation in the first devnet. Implement public request/response and bulk distribution behind a transport interface; avoid two incompatible mempools or duplicate gossip of the same transaction. Public nodes use multiple peers/relays. No mandatory master hub or global public-node-count limit is introduced.

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

An ephemeral non-voting public replica may be rebuilt. An ephemeral signing validator is forbidden. Restore of all RAM replicas must not depend on a single stale master copy that lacks recently finalized blocks.

## N07 — Admission limits

Initial local profile: 64 active peer connections per process, 8 concurrent bulk requests, 32 MiB in-flight bulk budget per peer, 1 MiB ordinary request envelope excluding declared chunk streams. These are local resource defaults, not consensus or public-node-count limits. Operator limits may tighten them without changing valid block semantics.

Keep dedicated quotas for `eth_call`, log scans, subscriptions, chunk requests and invalid signatures. Rate limiting by peer/IP is not claimed to solve Sybil resistance. Validate small headers before large allocation. On peer abuse, retain useful network diversity rather than disconnecting every independent route.

## Acceptance

T-N01: public node bootstraps from peers with master unavailable; every root matches.
T-N02: wrong-network, stale checkpoint, wrong validator set and wrong-height anchors fail.
T-N03: corrupted/missing/reordered/repeated chunks and deltas recover or reject deterministically.
T-N04: interrupted snapshot never destroys the live state; resume works after restart.
T-N05: master offline while blocks continue, then catches up byte-for-byte without voting.
T-N06: floods, decompression bombs, oversized manifests and slow peers remain within resource budgets.
T-N07: pruning refuses to destroy the last recoverable finalized history.
T-N08: adding public peers does not require one direct master stream per node.

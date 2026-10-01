<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public runtime

Canonical owner: public RPC, local admission, immutable query views, bootstrap and
public persistence orchestration. Validator components own execution and protocol
rules. This package has no private master dependency and grants no voting power.

The implemented B2 runtime is an explicitly acknowledged development producer.
It serves real HTTP/WebSocket RPC, accepts signed legacy/EIP2930/EIP1559
transactions for the frozen Shanghai profile, executes through the canonical
complete-state builder, and syncs complete state plus retained history before
publishing receipts or events. B2 integration acceptance is recorded separately
in `docs/execution/`; a compiling binary is not that acceptance.

`LOCAL_DEV_UNAUTHENTICATED` describes every locally produced block. Local execution
consistency and durability do not authenticate validator consensus. `latest`
selects the highest locally applied durable execution in this development profile.
`safe`, `finalized` and `eve_getFinalityProof` return `FINALITY_UNAVAILABLE`.
`eve_getNodeStatus` reports applied/durable heights and null authenticated/finalized
heights separately. Production and master-sync-only modes cannot activate this
producer. Classical account signatures are not post-quantum authentication.

## Development invocation and lifecycle

Use the pinned toolchain and a public genesis specification produced by the
[serial RPC acceptance client](../tests/acceptance/serial-rpc/README.md). The
specification contains public funded addresses and validator public keys. Client
signing keys stay in client memory.

```sh
cargo run --locked -p eve-public -- serve-dev \
  --root . --data local-tests/development-public \
  --genesis local-tests/public-development-genesis.json \
  --mode DEV_ALL_IN_ONE --acknowledge-unsafe-development \
  --http-address 127.0.0.1:8545 --ws-address 127.0.0.1:8546 \
  --block-interval-ms 1000 --zone-id 1
```

The supplied genesis file must already exist. Data must be a dedicated ignored
`local-tests/` descendant with no symbolic-link escape. A new namespace initializes
the exact validated genesis; reopening requires an exact match and complete
retained recovery validation. Old B1 history namespaces undergo bounded resumable
index bootstrap before listeners become ready. Startup prints one JSON object
with actual bound addresses, height, zone, verification mode and finality absence.
Port zero is supported for test listeners. External binding requires
`--allow-external-bind`. Interrupt signals stop only this runtime's listeners and
producer after any active durable transition finishes.

The library exposes `runtime::run_development_public(DevelopmentPublicConfig)`
for explicit development host composition. Master may call that public API; public
does not import master. Nonempty blocks use deterministic parent timestamp plus
one, zero development proposer and zero previous consensus hash, making identical
signed transaction sequences reproducible across independent local stores. These
inputs do not constitute consensus or finality. `zone_id` is typed routing metadata,
also synced in public namespace metadata; changing zone does not change genesis,
chain ID, authority or shard ownership.

## RPC and read semantics

Plan 18's required HTTP methods and WebSocket newHeads/log subscriptions are
implemented with real canonical data. Optional signing, admin, mining, tracing,
filter-polling and state-override APIs return method/parameter errors. RPC owns no
account, validator or master signing credential.

Each multi-key proof, log range and simulation uses one immutable captured version.
Independent JSON-RPC batch items may observe successive heads; pin an explicit
block number or supported block-hash selector for comparisons across requests.
Current reads check the service's RAM view first. Retained account state uses a
bounded two-view cache before repository loading. Historical blocks/logs use
narrow block projections, never a complete-state load for every scanned height.
Database handles remain private to the canonical recovery repository.

Pending account/call/proof queries execute one isolated actor-captured candidate
through the canonical builder without writing it. Pending nonce reports the
contiguous local sender chain. Pending block hash/number are null and the response
marks `evePending`. Pending historical log/fee queries return an explicit
`PENDING_HISTORY_UNAVAILABLE` error. Unknown transactions/receipts return null only
after the complete derived index proves absence. Unindexed, corrupt, stale and
unavailable history produces errors; RAM eviction is not pruning.

Storage slot input accepts full 32-byte DATA words and canonical minimal quantities.
Account/storage proof values bind the selected EVM root, with 256 requested slots,
duplicate rejection and 1 MiB actual proof data. They do not authenticate consensus,
a system root, an external bridge or Ethereum mainnet. Unsigned nonce and chain-ID overrides are unsupported: any caller-supplied
`nonce` or `chainId` returns -32602, including values matching the selected state.
Simulation uses the selected account nonce and canonical chain ID.
Calls and estimates preserve
selected block environment and canonical state; unsigned access lists have 256-entry /
1024-key caps. Successful gas estimates are checked by actual execution. Reverts
retain data, and simulation halts/errors remain explicit.

Fee suggestions use the parent-derived checked next base fee plus a one-base-unit
development priority default. Fee history uses actual retained headers, cumulative
gas and gas-weighted transaction tips; it does not query external price services.
Canonical execution preserves the owner's 40% burn/30% public-node pool/30%
validator pool policy. Native lifecycle activation remains B5; inactive native
calls revert under the canonical execution adapter.

## Admission and resource bounds

The actor owns mempool mutation and uses a bounded 256-command queue. Each admitted
raw transaction is at most 128 KiB, so queued raw admission bytes are at most 32 MiB.
Defaults are 10000 transactions, 64 MiB raw pool bytes, 64 per sender, future nonce
gap 64 and local monotonic TTL 300 seconds. Duplicates are idempotent. Replacements
raise each relevant cap by integer ceiling 10%, with a minimum one-base-unit rise;
type2 raises both maximum and priority caps. Cumulative maximum-upfront sender
reservation is a conservative local policy. It ignores possible incoming pending
credits until they commit. It is not an additional consensus rule.

Eligible sender heads are ordered by effective tip and hash while preserving
sender nonce order and local block gas/raw-byte bounds. Selection retains entries.
Only an acknowledged durable transition publishes a new actor head and revalidates/
evicts included or newly invalid entries. Failed or ambiguous sync fences fresh
reads/writes; it does not remove the leased candidate from the pool or publish a
receipt. Execution/local-capacity failure stops this development composition with
an explicit error, preserving the durable prefix for reconciliation.

HTTP: 1 MiB requests, 100 batch items, 4 MiB responses and 128 connections.
WebSocket: 64 KiB requests/responses, 64 connections, 16 subscriptions per connection
and 256 globally. Subscription IDs are opaque 16-character strings; unsubscribe
is scoped to the originating connection. Complete notification payloads reserve
2 KiB envelope/error margin.
Framework output queues hold 32 frames per connection: the conservative WS queue
budget is 132 MiB, separate from the clone/read quota. HTTP request/response buffers,
socket/OS buffers, database caches and allocator overhead are also separate from
that quota; no whole-process RSS cap or zero I/O overhead is claimed.
Persistent baseline state is also separately bounded: the current head/service,
mempool parent and up to two retained historical views (at most 64 MiB logical
state each) can outlive request leases. Their maps/allocator footprint is not the
same as encoded bytes. Pending sends and in-flight socket frames sit outside the
132 MiB queued-frame estimate; no total RSS or consensus-starvation result is implied.

Public worker ceilings are 128 active RPCs, 2 signature workers, 8 simulation workers,
4 general/history workers and 2 proof workers. A weighted 512 MiB logical clone/read
budget reserves 256 MiB for the serial development producer and 256MiB for public
queries/events. Cached Arc reads charge request/result buffers rather than copying
complete state. Misses reserve decode space before repository I/O. Simulations
add the canonical clone estimate and 32 MiB interpreter limit; proof reconstruction
adds its actual checked estimate. Memory requirements can lower effective
concurrency below those worker ceilings. A cancelled/timed-out blocking job retains
its permits until that job actually exits.

Log queries scan at most 1000 retained blocks, return at most 10000 events and obey
response bytes. Proof data is capped at 1 MiB. Shared immutable broadcast events hold
byte leases until the last ring/subscriber reference drops; subscribers do not
deep-copy a complete block's logs. Lag, event/frame capacity or slow output closes
the subscription with an explicit error and directs recovery to durable HTTP
history. Delivery to a stalled/disconnected peer is not guaranteed.

## Remaining production requirements

Authenticated validator finality, production P2P/relay, nearby eligible endpoint
discovery, independently authenticated bootstrap, master-offline tail recovery,
isolated asynchronous finalized persistence, retention/compaction and measured
regional/capacity acceptance remain later bulks. The development synchronous
repository path does not satisfy those production-worker gates or BFT starvation
evidence.

Standalone copy-ready distribution remains NOT_IMPLEMENTED/NOT_RUN: current
manifests use canonical sibling validator components. B6 must reproducibly package
exact owner sources, lockfile/toolchain, sanitized examples and notices, then build
and run after copying only this role directory to an unrelated clean location.
No private master implementation or unresolved parent path may remain there.

Read [plan18](../docs/plan/18-rpc-mempool-and-developer-experience.md),
[plan25](../docs/plan/25-folder-hierarchy-and-file-function-policy.md) and
[plan32](../docs/plan/32-regional-masters-and-public-persistence.md).

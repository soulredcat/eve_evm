# Public-owned development node policy

Canonical owner: `public/components/node-policy/`. This component owns public
resource/admission bounds, persistence/readiness watermarks and eligible source
policy. It depends one-way on validator-owned immutable protocol/profile contracts;
it has no master implementation, private database handle or consensus authority.
Source classification and readiness consume caller-provided verification metadata,
not authenticated capabilities. B4/B6 must implement the actual verifiers,
storage/network workers, atomic reservations and measurements.

Read plans [15](../../../docs/plan/15-network-and-sync-protocol.md),
[25](../../../docs/plan/25-folder-hierarchy-and-file-function-policy.md) and
[32](../../../docs/plan/32-regional-masters-and-public-persistence.md) before changes.

## Measured provenance and development bounds v1

The root B0 RocksDB spike measured four batches of 8 records of 256 payload bytes,
8192 total logical bytes. Sync samples were 95/61/51/51 microseconds; checkpoint
2772 microseconds; DB 123289 bytes; checkpoint 65685 bytes. WAL and sync were enabled.
GNU time reported 59348 KiB maxRSS for the Cargo-command/child workload, not an
isolated public worker/process cap. These were warmed Linux/WSL temporary-storage
API/reopen measurements, without finality, power-loss, interference or TPS proof.
Raw evidence is local-only `local-tests/b0-storage-measured.txt`.

The factory reproduces measured knobs: 4 MiB write buffer, count 2,4 MiB block cache,
2 background jobs,32 open files. The spike used 4 KiB records,64 KiB batches and
max 16 records; the larger 8 MiB record/batch ceiling below is an unmeasured
development policy accommodating complete block/recovery material. B4 must measure
it under real persistence, proof, replay and saturation work before acceptance.

| Bound | Development v1 |
|---|---|
| Record/batch | 8 MiB each,max 1 record/batch |
| Storage queue | 32 MiB,max 4 batches,oldest age 2000 ms |
| Working state | 256 MiB |
| Query cache/mempool/simulation overlays | 16 MiB each |
| Snapshot staging | 16 MiB |
| Bulk admission | 32 MiB per peer,64 MiB globally,max 8 requests |
| Ordinary admission | 1 MiB envelope,16 MiB globally |
| Process budget | 512 MiB target |
| Worker | 1 thread,2500 CPU basis points target,2 storage reads |
| Durable/authenticated/head lag | 8/1/2 blocks |
| Peers/probes | 64 connections,32 candidates,4 concurrent probes |
| Routing | 2000 ms deadline,30000 ms retry maximum/cooldown,15% RTT improvement |
| Source health | At most 10% measured request errors |
| Snapshot chunks | 4 MiB target,8 MiB decompressed maximum |

Configured pools sum 452 MiB, leaving 60 MiB allowance under the 512 MiB target. This
does not prove whole-process memory/CPU enforcement; allocator/database/OS caches,
thread stacks, sockets and shared-host interference still need measurement.
Validation rejects zero/inconsistent/overflowing limits and insufficient total
budget. Per-peer 32 MiB times 8 requests could otherwise permit 256 MiB of bulk buffers;
the independent 64 MiB global reservation prevents that declared policy overcommit.
The pure reservation check must run inside the caller's atomic reserve/release
protocol before buffer allocation; a check alone cannot prevent concurrent races.
These are development limits, not 1M-TPS settings or production-capacity inference.

## Watermarks and readiness

Finalized,applied,durable recovery,checkpoint and authenticated state have distinct
types. Durable state may lead applied state during recovery; readiness stays false.
Checkpoint cannot exceed durable coverage. Next-header post-state authentication
requires exact H+1 and an available finalized header; verified replay is a distinct
caller-provided mode. Snapshot/retention markers remain separately checked.

Readiness rejects storage failure, missing recoverable tail, excessive queue age/
bytes/count, durable/authentication lag, inconsistent markers or unknown independent
head freshness. A ready result advertises `serve_height=min(applied,authenticated)`
and separately returns durable height. It never serves an unattested H by silently
claiming the H+1 anchor exists, and a queued batch never advances durable height.
Historical reads/recovery and actual immutable-view capture remain runtime work.

## Source selection

Expected network/profile bindings must come from authenticated applicable-height
history. Source metadata must report a completed caller verification, matching
transport identity and bounded useful service metrics before ranking. This flag
does not itself verify anything. Unknown/unsupported profiles, wrong network,
invalid verification and unhealthy sources reject. Valid older history remains
eligible for replay while stale/unknown freshness excludes fresh-head use.

Rank eligible candidates by verified freshness, then error rate, service RTT,
useful throughput and stable identity bytes. Claimed height/ICMP ping is not a
trust anchor. Bounded hold-down and 15% measured RTT improvement avoid switching
for marginal differences; invalid current sources can fail over immediately to
an eligible replacement. Timer/probe scheduling is B6 work. `ZoneId` is routing
metadata and never changes network identity, voting power, finality or ownership.

## Verification and remaining gates

`cargo test -p eve-node-policy --locked` exercises declared allocation/admission
bounds, positive/negative watermarks/readiness, source freshness/proof/profile
classification, historical replay and hold-down cases. It does not satisfy
T-N09/T-N10 public persistence/crash gates or T-N11 actual network failover.
Run strict Clippy, format and the integrated structure/B0 gates before integration.
Copy-ready public distribution remains B6 work; the monorepo dependency path is
not a successful unrelated-directory role copy/build/run result.

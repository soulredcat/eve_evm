<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# T-N09 local storage interference measurement

The source contract is frozen in config/gates/measurement-b4.toml before execution.
The test parses it with deny-unknown-fields and verifies its declared profile.
Its 100 ms small-fixture p99 is a new CLASSICAL_DEV_LOCAL objective, not historical
performance evidence or secured aggregate throughput acceptance.

Run the exact case in a separate release test process:

    cargo test --release --locked -p eve-public sync::applied::segmented::tests::storage_pressure::measurement::t_n09_actual_checkpoint_io_and_paused_wal_keep_ram_rpc_bounded_under_frozen_local_contract -- --exact --nocapture --test-threads=1

Each phase has 128 rounds with two concurrent serialized JSON-RPC requests over
real loopback HTTP to the production server and RPC module: eth_getBalance and
eth_call. The simulation arena is the actual configured 8 MiB. Each request has
a 1 second timeout; each method's nearest-rank p99 must be at most 100 ms.
No direct dispatcher call is labelled network latency.

The pressure phase pauses the actual sole WAL writer before its first append.
Each round releases two ready storage actors alongside the HTTP requests: one
writes real canonical checkpoint chunks and proofs; the other performs actual
KEEP_ALL compaction and canonical transaction/block secondary-index lookups.
Separate private Linux archive namespaces preserve exclusive write ownership.
Both actors use the public owner's actual two-read controller and staging/working
leases. Each of the 128 checkpoint, compaction and index jobs must succeed with
unchanged retained payloads, cursors, versions, transactions and receipts.

Only one auxiliary database opens at a time. Its 64 KiB cache/write-buffer,
one background job and eight file slots are reserved from the existing combined
profile: the primary uses 4 MiB minus 64 KiB, one job and 24 file slots. Buffer
counts sum to the unchanged two-buffer budget. The auxiliary operation holds a
2 MiB working estimate and 512 KiB staging lease. Production defaults and aggregate
limits are unchanged. The contract fixes this allocation before measurement.
The shared fixture supplies canonical signatures and execution oracles; barriers
add no artificial disk sleep or claimed primary-WAL maintenance.

The test separately exercises read-slot and staging saturation before any artifact
write. It verifies immutable old/new views, pool peaks, exact charged queue bytes,
actual writer CPU pacing, shutdown and authenticated local reopen. If the paused
queue ages beyond its 2 second admission limit, a further ordinary application
must refuse and retain the existing charged tail; queue age is not a promise to
discard a paused write or force sync completion within 2 seconds.

Own-process /proc/self/status samples report the maximum observed RSS within the
scenario, separately from process-lifetime VmHWM. Samples do not prove an
unsampled peak, allocator containment, OS quotas or attribution of earlier tests.
Writer ThreadCPUTime covers paced record operations; database background threads,
RPC and checkpoint verification have distinct scope. The record burst and CPU
pacing limitations are documented in the production worker CPU capability.

Metrics output contains compact numeric measurements only. The root integrator
executes the case, reviews local raw output, and publishes the verified summary.
Source presence alone is NOT_RUN and does not close B4 or main integration.

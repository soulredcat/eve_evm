<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Cooperative bounded writer CPU pacing

The sole segmented WAL writer samples Linux thread CPU time through the safe
pinned rustix API. Invalid basis points, unavailable clocks, unsupported platforms
or arithmetic failures refuse before an unmeasured record operation proceeds.

Each bounded physical record operation starts a new CPU/wall window. After the
real compare-and-append operation, the writer sleeps for the positive remainder
of ceil(CPU nanoseconds × 10000 / basis points) minus elapsed wall time. Existing
IO wait counts toward elapsed time. Idle time between records never earns credit.
Pacing occurs outside publication, RAM and admission locks.

The development budget is 2500 basis points. One physical record can execute
before pacing, with the existing 4194304-byte payload ceiling. This is cooperative
operation-average pacing with a bounded record burst, not an OS scheduler quota
or instantaneous CPU ceiling. Clock/pacing failures after a real write remain
ambiguous storage failures and retain the charged worker tail.

Checked counters publish a bounded coherent observation of record operations,
measured CPU, elapsed wall, pacing sleep and the largest recorded CPU burst.
Failed compare-and-append attempts are still completed measured operations;
these counters do not indicate finalized transactions or durable heights.
Database background threads and public RPC/verification work remain outside the
writer pacing scope and require their own configured resource limits and tests.

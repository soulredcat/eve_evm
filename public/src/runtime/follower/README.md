<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Classical development public follower

`eve-public follow-dev` runs public-owned authenticated RAM application, isolated
segmented recovery persistence, shared mempool captures and the existing HTTP/WS
RPC implementation. It requires an explicit CLASSICAL_DEV acknowledgment, locally
configured genesis, loopback validator/listener addresses and a contained ignored
`local-tests/` data namespace. It starts no validator, master or local block producer.
The configured node name selects local namespace ownership and grants no voting power.

The runtime uses the unchanged development PublicBudget. RPC capacity combines its
16 MiB query-cache and 16 MiB simulation pools; the VM arena is explicitly 8 MiB,
with its possible copied output charged before execution. Mempool capacity is
16 MiB. Captures share the actual charged applied generation. Working, queue,
repository buffers, metadata and one worker scratch envelope retain their declared
limits. Accounting estimates do not establish allocator/RSS or CPU/IO isolation.

One bounded blocking actor polls actual ordered durable acknowledgements, downloads
the exact next validator delta and H/H+1 native data, and delegates canonical
verification/publication. All ordinary source failures leave the previous verified
view intact. Disk failure stops the actor; queued/partial tails remain available
through shutdown/recovery semantics. RPC captures remain independent of that IO.
Source selection currently uses one explicit endpoint, with no master inventory.

Current native JSON response admission is 256 KiB, delta chunks are 32 KiB, and
network deadlines are explicit. A valid larger remote response can therefore be
refused under this local profile. It does not change the canonical logical record
or consensus limits. Runtime success is not a global freshness assertion: readiness,
peer count and independently corroborated head remain unknown until their integration.
Certificate proof export, older RPC history, transaction relaying, authenticated
snapshot activation and integrated fault/retention acceptance remain required work.

SIGINT or SIGTERM stops new sync admission, joins the task-owned actor, reconciles
actual storage acknowledgements and closes only its own RPC listeners. No production
launch, PQ, hardware power-loss, secure throughput or complete B4 claim is made.
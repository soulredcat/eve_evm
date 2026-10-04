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
Certificate proof export, older RPC history, transaction relaying and integrated
fault/retention acceptance remain required work.

Checkpoint-aware local reopen is selected by default. An optional
`--checkpoint-height H` requests bootstrap before RPC listeners and normal follower
admission start, only when H exceeds the actual recovered applied height. It accepts
heights 1 through 10000 under finite local limits. It uses locally configured genesis;
a source's manifest, checksums or advertised durable tip grant no finality authority.

Generated checkpoint roots are `DATA/.checkpoints/content` and
`DATA/.checkpoints/proofs`. Explicit bootstrap creates and syncs owned private
0700 directories through Linux directory handles without following symlinks.
Ordinary startup with no base or bootstrap request creates no checkpoint directory.
Stored bases require exact completed artifacts and full genesis-to-H+1 verification
before publication; missing or invalid artifacts refuse startup.

Bootstrap reserves ingress against the actual owner's working pool. It downloads
32 KiB content chunks, skips intact staged chunks and checks one proof witness at
a time. The initial proof implementation downloads missing witnesses twice: the
first pass retains only bounded references and a stream checksum; the second pass
writes witnesses against those exact references. A changing source cannot rebind
the staged manifest. Full canonical proof verification still follows both passes.

The whole bootstrap has a 300-second deadline, with each network operation limited
to five seconds or the remaining whole deadline. Content is capped at 32 MiB,
content manifests at 256 KiB, witness history at 128 MiB, and startup scanning at
100000 actual physical rows. These local limits preserve the existing 256 MiB
working estimate and can refuse larger valid inputs. They are not consensus limits,
allocator/RSS measurements, bandwidth efficiency or throughput acceptance.

Activation uses the existing sole worker's real synced base acknowledgment and a
conditional immutable RAM publication. Interruption preserves staged data and the
previous published view until activation. Failure joins this runtime's storage
worker before returning. A deadline after accepted base submission may leave a
complete valid base that must be authenticated again on the next startup. Kernel
IO cancellation and hardware power-loss guarantees are not claimed.

SIGINT or SIGTERM stops new sync admission, joins the task-owned actor, reconciles
actual storage acknowledgements and closes only its own RPC listeners. No production
launch, PQ, hardware power-loss, secure throughput or complete B4 claim is made.

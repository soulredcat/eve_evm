<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public checkpoint process interruption

This T-N04 development case uses four actual validator processes and the actual
release public CLI. A first signed transfer is durably followed before shutdown.
A separate default-startup consumer with an unavailable source captures the
exact drained old height, cursor, roots, block and account state through the
canonical runtime and existing independent replay oracle.

Validators then finalize a second signed transfer while the public process and
all masters are absent. An owned loopback TCP proxy forwards opaque native RPC
bytes with fixed buffers and finite byte/connection/time bounds. The actual
checkpoint body must fit one 32 KiB content chunk. Connection one obtains the
manifest and connection two obtains that chunk. Connection three is paused
before forwarding the first proof-metadata request. The test asserts actual
content completion, then identity-checks, SIGKILLs and reaps its own public child.

The proxy is cancelled and joined; no shared RPC parser, signature code or
protocol message implementation is copied. After reaping, the canonical completed
content reader compares the whole stored commit, including code, transactions
and receipts, with the independent execution oracle. A default restart with the
source unavailable must restore exactly the old roots and cursor with checkpoint
height zero. The old durable state is never inferred from a queued acknowledgment.

The same checkpoint target and data directory then resume against the actual
validator endpoint. Startup must report the exact authenticated target; runtime
status must report a nonzero actual durable base cursor and correct checkpoint
markers. Current public state is compared with canonical replay, including nonce
two and recipient balance. The completed content identity and exact bytes remain
unchanged after activation.

There are exactly five owned public launches across three separately constructed
consumer fixtures. Each retains the existing two-launch ceiling; no counter is
reset. Every child uses the existing executable/start-identity cleanup helpers.
The proxy owns at most one accepted connection and two relay directions, 8 KiB
buffers, 1 MiB request and 256 KiB response caps, a 15-second connection deadline
and a 180-second total listener deadline. Its cancellation path joins its threads.
The two inspection listeners deliberately supply no source response and forward
no data; they reserve distinct loopback endpoints without host networking changes.

This is process-interruption evidence after content sync and before proof/base
completion. It is not a hardware power-loss test, a fault at every persistence
boundary, proof of independent failure domains, PQ acceptance or complete B4.
The integrator must register and execute the case before reporting a pass.

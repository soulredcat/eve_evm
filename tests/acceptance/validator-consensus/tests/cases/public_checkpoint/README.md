<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Actual public checkpoint bootstrap, online tail and default restart slice

This T-N04 development slice uses four actual native validator processes and the
built release public product. It starts no master. A nonempty nonce-0 transaction
is finalized, and public bootstraps at that execution height through the actual
checkpoint transport, content/proof stores, native verification and synced base
activation. Startup must identify the requested authenticated height. RPC must
report its actual checkpoint, authenticated snapshot and durable watermarks.

The fixture uses a private Linux-native temporary Git root, with only its owned
`local-tests/` namespace ignored. This preserves actual directory sync and atomic
no-replace requirements on filesystems that support those operations. The actual
validator genesis, public executable, configured native source, process identity,
launch limits, private output bounds and shared RPC framing remain unchanged.

After bootstrap, coherent current RPC roots, sender balance/nonce and recipient
balance match independent native history replay. The actor may already have
advanced beyond the checkpoint height; current RPC does not promise historical
receipts. Another real nonce-1 transaction finalizes while public runs. Its actual
RPC receipt and complete current state match replay, and tail persistence keeps
the original checkpoint markers. Unknown freshness remains not-ready.

The owned public child is identity-checked, SIGKILLed and reaped. Only after it is
gone, canonical completed-store APIs reconstruct the immutable checkpoint body.
Exact canonical bytes, complete state, roots, fees, nonce and the original raw
receipt match the independent checkpoint-height oracle. This independent reader
uses a real test-only 128 MiB logical reservation pool, a 4 MiB body bound and
canonical actual-count decode requirements. It cannot inspect or establish the
child runtime's pool, allocator usage or RSS.

Restart uses the same data and omits `--checkpoint-height`. Actual base/tail
reopening must preserve authenticated/durable progress and checkpoint markers,
and current replay comparison must retain sender nonce exactly 2. After graceful
shutdown the completed checkpoint is reopened again and its identity must remain
unchanged. Source, fixture history and replay are bounded to 128 execution heights;
startup allows 180 seconds for the requested bootstrap and 90 seconds on restart,
within the product's separate 300-second bootstrap limit.

This is actual single-host classical checkpoint bootstrap/activation, online tail
and process-restart evidence only after the integrator executes the frozen case.
Interrupted transfer, hardware power loss, independent-host availability, fresh
head readiness, PQ protection, throughput and complete B4 acceptance remain
separate requirements. This source file records no passing execution result.

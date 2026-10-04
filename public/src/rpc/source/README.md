<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# RPC state sources

RPC uses a real durable StateService/StateReader or an AppliedReader. Current RAM
queries capture one immutable AppliedPublication and retain its charged generation
through state reads, VM work and proof construction. No dummy durable namespace,
uncharged StateCommit Arc or certificate capability is constructed for RAM reads.

The development context factory preserves its existing limits and behavior. The
applied factory requires an explicit local byte and VM-memory profile and reserves
no producer capacity. Its KiB-accounted byte profile must be positive and aligned;
overflow and unavailable worker/byte capacity reject before work. Applied VM leases
cover the configured arena and possible copied RETURN/REVERT output. Both modes
retain active, worker, signature and byte permits through blocking work, including
caller cancellation. These are logical reservations, not allocator/RSS guarantees.

Applied history currently contains the captured block only. Number/hash selectors
must identify that same block; older ranges return GAP and unapplied/pending state
returns NOT_READY. Unknown transaction/receipt history is not fabricated as null or
PRUNED. Applied history jobs reserve from actual captured payload/container sizes
and use that same publication through encoding. Durable history remains unchanged.
Both sources share canonical block, receipt and log encoders through borrowed views.

Genesis is locally trusted and has no certified application outcome. Positive
applied views expose their actual H/H+1 authentication and explicit import/replay
mode. Node status distinguishes applied, durable and authenticated heights, known
physical cursors, storage failure and missing recovery range. Head freshness, peer
count and head lag remain unknown in this source foundation; a valid historical
anchor cannot grant readiness. Certificate-proof export remains explicitly absent.

`eve_getStateRoots` takes no parameters and returns one captured view's height,
EVM/system roots, execution hash, content digest, nullable application commitment,
verification mode and actual anchor presence. The height is a canonical hex quantity;
hashes are 32-byte hex values. A locally consistent application commitment alone
cannot set authenticatedFinality. contentDigest includes auxiliary local data and
is not separately certified by EVE_APP_V1, even for an authenticated imported view.

The listener interface accepts actual HTTP/WebSocket addresses while preserving
existing development callers. Root runtime orchestration owns listener lifecycle,
transaction relaying, independently verified head observations and readiness
integration. This source layer alone does not complete B4 or secure-profile gates.

Tests reuse the existing real signed import fixtures and actual storage worker
pause/refusal. They cover genesis labels, RAM queries during paused append,
unavailable history, source-generation leases, explicit VM/proof pressure and
truthful failed-storage markers. Verification results belong to recorded root gates.

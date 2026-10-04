<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Complete configured public replica loss and durable validator recovery slice

T-G06 starts two actual release public followers against the existing four real
validator processes, with no masters. Each public follower owns a separate private
Linux-native temporary Git root, separate listener ports and contained ignored
data. Both authenticate and durably retain the first nonempty transaction.
Coherent current roots, EVM/system/application commitments, sender balance/nonce
and recipient balance match independent canonical native history replay.

Both task-owned public children are executable/PID-start checked, SIGKILLed and
reaped before their configured data directories are relocated. Relocation verifies
absolute targets within each exact private fixture root, normal owned directories,
an unchanged held parent and an absent target. Atomic no-replace rename preserves
the original directory and all its bytes under an unused generated name. This
test performs no recursive deletion, overwrites no existing entry and touches no
production data. Only the disposable TempDir owners clean their namespaces at exit.

This is explicitly `SIMULATED_CONFIGURED_REPLICA_DATA_LOSS`: every configured public
replica's data path becomes absent, while preserved bytes remain on the test host.
The relocated copies are never configured, read for recovery or returned through
the network. Metadata-only assertions show they remain preserved and distinct
from newly rebuilt directories. This is not worldwide loss of every physical copy.

Validators finalize another nonempty nonce-1 transaction while all public followers
and masters are absent. Both public products then restart using their original
arguments and empty configured paths. Startup must report genesis height zero and
no authenticated finality. Each subsequently authenticates the retained own-chain
validator data and persists a complete new local prefix through the later transaction.
Current complete roots and account state match independent replay, sender nonce is
exactly two, and the original certified prefix is unchanged. Fee effects remain
bound by the exact sender balance and complete authenticated state roots.

The public RPC currently serves receipts only for its current immutable publication.
This slice compares coherent complete roots and account effects rather than claiming
old receipt availability after either follower advances. Applied, authenticated,
finalized and durable markers must remain distinct; actual marker/physical ACKs
are checked. Checkpoint markers stay zero because no snapshot was requested.
Unknown freshness honestly remains `ready=false`.

The fixture uses at most two launches per public node. Shared startup and durable
wait budgets are 90 seconds, current oracle comparison budgets are 30 seconds per
replica, and transaction/replay execution heights are bounded to 128. Existing
private output and actual RPC framing/deadline limits are reused without change.
It copies no consensus, signing, HTTP parser or execution implementation.

The integrator must execute this frozen case before recording a pass. Actual node
processes, real validator retained data and own-chain recovery are exercised only
by that run. Single-host process loss and simulated configured-data loss do not
establish hardware power-loss durability, geographic availability, loss of every
global copy, PQ protection, fresh-head readiness or complete B4 acceptance.

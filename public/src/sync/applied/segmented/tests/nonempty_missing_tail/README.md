<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Nonempty authenticated tail-loss source fixture

This test domain labels the fault SIMULATED_UNSYNCED_LOSS. It uses the canonical
shared recovery corpus with a signed nonce-zero transaction at H1, signed
nonce-one transaction at H2 and certified H3 lookahead. It does not copy the
executor, native signer, transaction parser or cryptography. One DTO adapter uses
maintained journal projection, finality import and V2 wire codecs.

The actual public worker syncs H1 data and marker at physical rows 1/2. It then
applies nonempty H2 in RAM and syncs H2 data at row 3; the existing test-only pause
and panic occur before its marker. Only those actual acknowledged rows are copied
to a new real WAL namespace. The original database and its fully charged failed
tail are preserved. This models loss of the unsynced marker without claiming
hardware power loss, physical flush behavior or a process-kill experiment.

A separate explicitly selected peer/validator-source WAL stores canonical complete
commits and immutable certified-input wires through actual WAL/sync acknowledgment.
The fixture wire has already passed canonical H/H+1 import against locally
configured genesis. Peer record integrity itself grants no finality or freshness.
Actual bounded reads acquire the public service's real working reservation before
allocation and retain it through decoding/application. Canonical commit decode
uses sealed state preflight and a separate actual decode reservation.

Recovery reconstructs only H1 from the copied complete prefix, exposes missing
height 2 and NOT_READY, then reads the real peer WAL H2 wire and reauthenticates it.
The whole H2 commit, receipts, nonce-two state and complete fee/system material
match the canonical execution oracle. Repeated delivery is refused, the old held
view remains coherent, actual ordered durability reaches H2, and reopen restores
the same complete state.

The unavailable branch selects a peer WAL with no H2 wire. The corrupt-only branch
selects only a shaped wire with invalid H3 authentication. Preserved original
databases are outside that selected source set; this fixture does not claim global
loss of all data. Both branches retain the exact H1 state/fees/cursors,
missing-from height 2 and NOT_READY through reopen. Thirty-two repeated actual
peer reads and malformed/stale inputs retain baseline working leases, zero pending
queue and zero retained parts. This is bounded rejection evidence, not a complete
hostile-network/RSS or production fault-model acceptance claim.

No master process is started by these tests. The separate real four-validator
native-query/process test covers actual durable peer transport; combining these
scopes does not turn the simulation into a physical-power-loss result. The
integrator must run this source and all bulk gates before claiming verification.

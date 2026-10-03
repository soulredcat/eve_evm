<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public recovery application

Canonical owner: public runtime. This directory owns local recovery admission,
immutable RAM publication, resource leases and interpretation of actual synced
storage acknowledgements. Validator-owned finality-verifier authenticates native
history and performs canonical execution replay. Public code does not decide
finality, import private master behavior or copy the EVM implementation.

The applied service has explicit EmptyReplay and AuthenticatedImport modes.
The legacy constructor preserves empty execution/lookahead admission and its
existing namespace. Import authenticates reconstructed nonempty outcomes at H/H+1
without REVM; private imported capabilities never substitute for independent replay.
See the [applied contract](applied/README.md) for actual leases and namespace binding.
Both are local B4 implementation slices; full runtime/recovery acceptance remains.
Encoded input capacity is reserved before copying or decoding. Working-state
charges use checked conservative logical estimates and remain held by captured
immutable views. They do not prove allocator capacity, physical RSS or OS memory
enforcement. Legacy replay default StateBudget maxima do not fit the public
256 MiB working pool. Import sizes actual genesis/candidates before allocation; local
test profiles do not establish default production capacity.

One owner prepares outside the publication lock and submits immutable bytes to
the exclusive bounded record worker. Applied/authenticated progress and admitted
cursors must remain separate from durable progress. Only an exact ordered actual
synced acknowledgement advances the durable prefix. Pending payloads retain their
charge and ownership after storage failure, including explicit shutdown results.

Open/restart validates the actual repository identity and replays its complete
retained prefix before exposing the recovered view. Master acknowledgements are
absent from this local path. Fresh peer head, missing-tail retrieval, snapshots,
deltas, fragmented records, retention and live public/master integration remain
separate required B4 work. No full T-N09/T-N10, B4, PQ or throughput acceptance is
claimed by this directory's API or isolated unit tests.

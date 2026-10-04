<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Existing checkpoint-base recovery

Public startup explicitly chooses checkpoint-aware recovery with locally
configured private content/proof roots and a finite physical-row scan limit.
The existing constructor retains its genesis/marker-only guard. With no typed
base, checkpoint-aware startup uses that same genesis recovery and does not open
or create artifact directories.

Startup scans one bounded actual opaque row at a time, checking the continuous
physical parent chain. Each selected base is re-read as actual repository
membership; its canonical target and metadata are framing/equality inputs rather
than authority. A malformed latest base, broken chain, missing completion,
corrupt file or exceeded local scan budget refuses startup before publication.

Exact completed snapshot and proof namespaces are reopened through private
directory handles. Metadata, IO, target decoding and retained membership have
real owner working-pool leases. Complete checksums still require the canonical
genesis-to-H+1 proof stream and complete target validation. Base target, binding,
snapshot/proof identities, stream hash and retained-range metadata must all match
the actual authenticated candidate.

The explicit checkpoint scanner then imports every complete V2 suffix through
the canonical H/H+1 verifier. Actual incomplete tails remain missing ranges.
Worker startup accepts the separately validated base membership if no later
complete marker exists; later markers use the existing verified-marker guard.
No synthetic H/cursor-zero view or second WAL is introduced.

Published applied, durable and authenticated heights reflect the recovered
prefix. Checkpoint and authenticated-snapshot markers retain the actual base H.
All history remains retained from zero in this profile. Fresh-head readiness,
hardware power-loss guarantees, measured resource interference and complete B4
acceptance remain separate requirements.

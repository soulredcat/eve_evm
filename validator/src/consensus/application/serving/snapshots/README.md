<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Native development checkpoint query serving

Validator application owns this read-only `/eve/recovery/v1/checkpoint` ABCI query.
It exports untrusted checkpoint manifests/chunks and execution metadata, using
public recovery storage's canonical message/content codecs. It never creates
native certificates, votes, signatures, new state or master-dependent behavior.

Every request binds the exact locally configured genesis StateVersion. Target
height must be positive and retained. A query captures current StateService RAM
cache first; database head reload occurs only when cache is missing, after the
configured decode ceiling is admitted. One actual StateRepository snapshot must
match that captured current version exactly. Current target uses the existing
immutable cache; historical target is read only from that snapshot, after observing
and charging its actual retained encoded length before decode.

One captured target supplies body, target metadata, manifest and every returned
chunk. Body uses canonical StateCommit encoding and sealed preflight before the
maintained checkpoint manifest builder. Manifest ID is deterministic for that
exact target and requested width. Each chunk regenerates the same content and
refuses a different manifest ID before releasing data. The durable tip identifies
the captured local database view and grants no independent fresh-head authority.
Execution queries return only canonical target version and BlockPayload; clients
obtain actual native frames separately and validate ordered H/H+1 witnesses.

The explicit development budget admits at most 32 MiB body, 4 MiB chunk,
262144-byte manifest and 1 MiB request under the existing 64 MiB serialized working
query ceiling. Actual current/historical state and canonical work are charged
conservatively at 128 times encoded bytes plus bounded scratch before historical
decode, cloning or body encoding. Encoding, manifest metadata and returned chunk
copies have further checked allowances. The exclusive application actor permits
one active query and holds no new global RAM-state lock during storage reads or
snapshot encoding. Limits are logical admission, not measured RSS or zero
CPU/I/O interference. A cold/default-max or otherwise large valid target may
correctly return RESOURCE_LIMIT without raising this budget.

Errors reuse existing EVE_RECOVERY codes: 1 WRONG_NETWORK, 2 UNSUPPORTED_VERSION,
3 GAP, 4 RESOURCE_LIMIT, 5 NOT_READY, 6 MALFORMED_REQUEST. Fencing, incoherent
cache/snapshot versions, unavailable content and wrong IDs do not fabricate state
or reset storage. Compression and proving queries are unsupported.

Source tests delegate the existing real durable application fixture and cover
current/historical deterministic export, exact body reconstruction, execution
metadata, held cache/database snapshot consistency, fixed errors, working and
transport limits, and cold-cache refusal before database reload. The integrator
must run these tests and publication gates before recording verification.
This single-host classical development capability does not establish production
transport, public checkpoint activation, physical power-loss behavior, PQ
acceptance, complete B4 or throughput performance.

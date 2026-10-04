<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Public validator-source follower

Public owns the one-next-height download/admission adapter. It captures the actual
charged RAM publication, requests a delta against its exact parent version and
acquires actual service-pool working leases before transport, assembly and decoded
materialization. The leases survive all corresponding owned values. Declared
numeric client estimates alone never acquire resources or establish authority.

The source client verifies bounded ordered chunks and whole-body transfer identity.
It obtains the actual native block/certificate at H and H+1, preserving lookahead
Data.Hash inputs. H transactions must equal the delta execution transactions.
The adapter encodes EVE_IMPORT_V2 and delegates candidate/resource sizing, H/H+1
verification and atomic admission/publication to the canonical applied service.
Advertised target metadata is an untrusted equality constraint checked against the
locally verified candidate before enqueue; it never authenticates state itself.

Master storage, source timestamps, claimed heights and chunk hashes are absent from
consensus authority. Existing finalized/applied/durable/authenticated markers keep
their distinct meanings. This adapter does not infer fresh-head readiness from a
valid historical outcome or an advertised durable tip. The current source is an
explicit loopback validator endpoint under CLASSICAL_DEV. Public-peer serving,
source fallback, runtime scheduling, snapshots and full integrated recovery evidence
remain required. No production TLS, PQ, physical power-loss or TPS claim is implied.
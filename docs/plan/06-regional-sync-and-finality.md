<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 06 — Regional synchronization and finality

## First regional deployment

Begin with one master and one public runtime plus the four-validator devnet baseline. A single-validator harness is local-only development. Introduce `zone_id` immediately for routing and operational failure domains; keep it distinct from network identity, immutable genesis hash and EVM chain ID. Zones do not create independent voting authority, state ownership or finality.

Expand public ingress and validator peers across regions while retaining one logical EVM state and one consensus history. One master can supply multiple public nodes. Masters evolve from one follower to two mutually synchronized independent finalized-history replicas, then toward ten regionally placed masters. The expansion is planned topology, not evidence of deployment, purchased capacity or launch approval.

Nearby RPC reduces the user's network hop to ingress. It does not eliminate the propagation, data availability and voting latency needed for global finality. Bulk/pipelined communication amortizes per-message overhead without removing network bandwidth or consistency requirements.

## Nearby source selection

Each public node uses one preferred nearby logical sync endpoint or relay without requiring internal master inventory or privileged access. Check source identity, network/genesis, supported protocol and active security profile, proof eligibility, authenticated lag and retrievable data before ranking eligible sources by measured service latency, useful verified-data throughput and recent reliability. ICMP ping is only a possible reachability observation and cannot establish those properties.

Use bounded probes/retries and switching hysteresis with fallback to eligible endpoints or peers. A source change preserves the verified base, staged-import safety and network-bound resume cursor. The preferred sync connection does not replace diverse P2P routes for live finalized data, transaction relay or master-independent recovery. Network isolation and relays reduce exposure but do not promise anonymous masters.

## Bulk data model

Bootstrap from an authenticated snapshot, then apply finalized blocks/deltas in order. Each logical batch has a base height/root, destination height/root, data commitment, size limits and consensus provenance. Transport may split it into many chunks. A commitment is not a compressed copy of the underlying data.

Matching a delta's computed root proves consistency with that root, not that the root is authorized. Authenticate the root to validator history, or independently replay transactions from a trusted state.

Each master verifies finalized provenance independently, including data received from another master. Mutual sync transfers authenticated history, snapshots and ordered deltas; it does not merge writable EVM states. Keep actual finalized, applied, durable and authenticated-snapshot heights distinct from source-advertised progress.

Public working state stays RAM-heavy, with verified blocks/checkpoints and recovery metadata durably stored by default in an isolated bounded path. Recover from the last complete authenticated local recovery point, retrieve and verify gaps, then restore readiness. Disk-pressure handling must preserve the recovery/availability limits in plan 15 and never claim data durable before persistence completes.

## Failure behavior

A master outage does not stop consensus while validators retain state/data and quorum. A validator quorum outage stops new finality. Storage replication failover does not elect a block producer. A network partition must not create two final histories under the stated fault threshold.

Masters in different countries may independently mirror the same finalized history. They do not need a global writable database lease unless sharing a single mutable storage namespace. Each local namespace needs single-writer safety; snapshot publication needs atomic manifests and fencing where appropriate.

## Later partitioning

Actual sharded execution is gated research, not the default meaning of regional replication. Specify state ownership, cross-domain atomicity, data availability, committee security, migration and failure recovery before enabling it. Independently executed conflicting batches cannot be merged merely by hashing their roots together.

[15](15-network-and-sync-protocol.md) defines sync, [12](12-consensus-spec.md) finality, [21](21-capacity-and-regional-scaling.md) the capacity/sharding gates, and [32](32-regional-masters-and-public-persistence.md) the regional-master/public-persistence requirements. No regional runtime or acceptance result is claimed by this document.

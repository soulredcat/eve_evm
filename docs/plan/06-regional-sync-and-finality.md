# 06 — Regional synchronization and finality

## First regional deployment

Operate public ingress and validator peers in several regions but keep one logical EVM state and one consensus history. Region/node IDs identify routing endpoints and failure domains; they do not authorize conflicting writes to the same account or pool.

Nearby RPC reduces the user's network hop to ingress. It does not eliminate the propagation, data availability and voting latency needed for global finality. Bulk/pipelined communication amortizes per-message overhead without removing network bandwidth or consistency requirements.

## Bulk data model

Bootstrap from an authenticated snapshot, then apply finalized blocks/deltas in order. Each logical batch has a base height/root, destination height/root, data commitment, size limits and consensus provenance. Transport may split it into many chunks. A commitment is not a compressed copy of the underlying data.

Matching a delta's computed root proves consistency with that root, not that the root is authorized. Authenticate the root to validator history, or independently replay transactions from a trusted state.

## Failure behavior

A master outage does not stop consensus while validators retain state/data and quorum. A validator quorum outage stops new finality. Storage replication failover does not elect a block producer. A network partition must not create two final histories under the stated fault threshold.

Masters in different countries may independently mirror the same finalized history. They do not need a global writable database lease unless sharing a single mutable storage namespace. Each local namespace needs single-writer safety; snapshot publication needs atomic manifests and fencing where appropriate.

## Later partitioning

Actual sharded execution is gated research, not the default meaning of regional replication. Specify state ownership, cross-domain atomicity, data availability, committee security, migration and failure recovery before enabling it. Independently executed conflicting batches cannot be merged merely by hashing their roots together.

[15](15-network-and-sync-protocol.md) defines sync, [12](12-consensus-spec.md) finality, and [21](21-capacity-and-regional-scaling.md) the capacity/sharding gates.

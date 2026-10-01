<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 01 — System architecture

Status: documentation requirements only; runtime deployment and recovery behavior are not implemented or verified yet.

## Authority map

| Role | Owns | Must not own |
|---|---|---|
| Public | RPC, local mempool policy, peer propagation, RAM working state, durable recovery data, verified queries | Production votes without validator registration; master credentials |
| Validator | Proposal validation, execution, consensus votes, finality, recent durable recovery data | Unilateral finality; master administration |
| Master | Verified finalized-state persistence, snapshots, archive, recovery distribution | Mandatory transaction sequencing, production proposer key, override of quorum |
| Role-owned reusable components | Deterministic formats, EVM adapter, roots, verification, storage contracts | Hidden role authority or cross-runtime globals |

One operator may co-locate public and validator processes. Source separation remains mandatory. A master host may run a separate local development validator, but that does not make the master role a voter.

## Required flow

```text
transaction -> public/P2P -> validator proposer -> validator checks and quorum
            -> finalized data -> peers and master followers
```

Validators do not wait for master acknowledgements in ordinary consensus. They must retain sufficient durable blocks/state to recover while masters lag. Masters may use fast authenticated state import or slower replay auditing; those are follower verification choices, not authority.

## Boundaries

Use separate traffic budgets for public RPC, consensus, block availability, bulk snapshot/delta transfer, and management. Sharing a physical host does not justify shared unrestricted credentials or unbounded queues. Consensus and management are not routed through a single mandatory master gateway.

Core components contain no external-chain route registry, custody verifier or remote-chain client dependency. Versioned EVE headers, receipts, finality evidence and proof interfaces remain native verification/recovery capabilities. Future separately authorized adapter programs consume those public interfaces and submit ordinary authenticated EVE transactions; they cannot require remote-chain availability, confirmation speed or acknowledgement before EVE proposes, votes, finalizes or durably records a block. External work is `DEFERRED_UNTIL_EVE_TESTNET` under decision D40 and plans 28/30.

## Baseline and evolution

Start with one master and one public runtime, together with the four distinct equal-power validators required for the consensus devnet. An earlier single-validator or all-in-one harness is local development only and cannot satisfy distributed-consensus acceptance. One master may serve several public nodes through bounded object distribution or relays.

Define `zone_id` from the first deployment as an operational routing/failure-domain identifier. It is distinct from network name, immutable genesis hash and EVM chain ID. A zone neither owns independent EVM state nor grants voting power or finality. Zone changes must not alter chain identity.

The first devnet uses one logical ordered EVM state and a reviewed BFT adapter. Parallel execution must match serial results. Actual state sharding is a later experimental protocol change with its own atomicity and security gates.

Evolve master storage from one follower to two mutually synchronized, independently verifying finalized-history replicas, then toward ten masters placed across planned regions. Each master owns a separate local storage namespace and validates validator history and commitment binding even when another master supplies the data. Replication does not merge concurrent writable chain states, add voting power or prove higher finalized TPS. Ten masters is a deployment direction, not a hardware purchase or mainnet launch authorization.

## Public synchronization and persistence

A public node normally uses one preferred nearby logical synchronization endpoint or relay. It does not need the internal master inventory, database location or privileged credentials. Transaction relay, live finalized-block P2P distribution and validator consensus remain master-independent; a preferred bulk endpoint is not the node's only peer or recovery route.

First check authenticated source identity, correct network/genesis, compatible protocol/security profile, proof eligibility, verified lag and retrievable data. Only eligible sources are ranked by measured service latency and useful verified-data throughput. ICMP ping alone cannot establish eligibility or synchronization quality. Use bounded fallback discovery, retries and switching with hysteresis; endpoint hiding is not an anonymity guarantee.

Public working state remains RAM-heavy, with durable verified blocks/checkpoints and recovery metadata enabled by default in an isolated, capacity-bounded storage path. Report actual applied, durable and authenticated heights separately. Persisting received bytes does not make an unverified snapshot usable; never discard the only recoverable finalized copy to maintain a storage or TPS target.

The selected implementation baseline and dependency policy are in [22](22-code-layout-and-dependency-policy.md). Consensus and failure behavior are normative in [12](12-consensus-spec.md). Regional masters, endpoint selection and public persistence are specified in [32](32-regional-masters-and-public-persistence.md). All role APIs use versioned protocol data rather than direct database access.

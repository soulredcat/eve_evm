# 10 — Master, public and validator operating model

## Final architectural decision

**Validators decide; master remembers.** This supersedes the earlier exploratory idea that master can finalize blocks when validators are unavailable.

## Three source roots

`master/`, `public/`, and `validator/` are separate runtime entry points consuming shared crates. Public/validator distributions must compile without depending on master implementation. Co-location is optional and does not collapse role permissions.

## Operating profiles

| Profile | Meaning |
|---|---|
| DEV_ALL_IN_ONE | Local-only composition of public RPC, a test producer/validator and durable storage; development keys and genesis only |
| MASTER_SYNC_ONLY | Developer-operated follower importing verified finalized history; no production vote/proposer key |
| PUBLIC | Permissionless RPC/P2P/verified-state replica without voting power |
| VALIDATOR | Registered signer/executor/consensus participant with durable signing records |
| PUBLIC_WITH_VALIDATOR | Co-located public and validator roles with separate credentials and resource limits |
| MASTER_REPLICATED | Multiple followers or local primary/standby storage, without additional consensus authority |

The historical MASTER_ONLY name refers only to the DEV_ALL_IN_ONE development harness. Production configuration must reject enabling its local producer against a production genesis. A single-operator devnet is not a decentralized deployment.

## Initial topology and master expansion

Start with one master and one public runtime alongside the four distinct equal-power validators in plan 12. An earlier single-validator or all-in-one harness is local development only. It neither replaces the four-validator acceptance topology nor establishes decentralized security. A master may serve several public nodes through bounded object APIs and relays.

Use `zone_id` from the first deployment for routing and operational failure domains. Network name, immutable genesis hash and EVM chain ID retain separate meanings. A zone does not own independent EVM state, assign voting power or change consensus finality.

The master sequence is one follower, then two mutually synchronized independent durable replicas, then a planned ten masters across regions. Each verifies validator finality, set history and state-commitment binding independently even when another master supplies data. Use separate local namespaces and prohibit writable chain-state merging; single-writer fencing applies when processes share a mutable namespace, without a global writable-database lease across independent masters. Replication is not increased consensus authority, decentralized ownership or measured TPS. This direction does not authorize purchases or launch.

## RAM and recovery

The default public mode combines RAM-heavy working state with durable verified blocks/checkpoints and recovery metadata in an isolated capacity-bounded storage path. Report actual applied, durable and authenticated-snapshot heights and rebuild from the last complete authenticated recovery point after RAM loss. An explicitly configured ephemeral non-voting mode remains a separate recovery choice; it must fully reverify after loss and cannot be advertised as the durable default.

Validators may keep hot state in RAM but must durably preserve signing safety, recent blocks and recovery state independently of masters. Masters keep durable data with a measured fsync/recovery policy. RAM or OS page-cache contents are not durable evidence. Storage budgets and pruning must preserve the only recoverable copy of finalized history.

## Failure table

- Master offline: finality continues only while validator quorum and required data remain available; master catches up later.
- All RPC public nodes offline but validators connected: consensus can continue; user ingress may be unavailable.
- Validator quorum unavailable: new finality stops; master preserves the previous finalized history.
- Corrupt master/snapshot: reject unauthenticated or mismatching data; obtain it from another source.
- Signing-key state lost: validator does not resume blind signing; follow the fencing/recovery runbook.

## Access and updates

Public participation is not bounded by a protocol-wide node count, but peer connections, bandwidth and active consensus membership are bounded independently. Use P2P and relays so every public node need not connect to master.

A public node normally selects one preferred nearby logical sync endpoint or relay without internal master inventory or database/admin credentials. Source identity, network/genesis, compatible protocol/security profile, historical proof eligibility, authenticated lag and retrievable data are eligibility checks. Rank eligible sources by measured service latency, useful verified-data throughput and recent reliability; ICMP ping alone is insufficient.

Bound source discovery, probes, retries and switching; use hysteresis/cooldowns and another eligible endpoint or peer on failure. Preserve the verified base and resume cursor. The preferred bulk endpoint is not the only transaction or live-block P2P route, and its failure cannot grant master authority or halt an otherwise healthy validator quorum. Endpoint hiding reduces exposure without guaranteeing anonymity.

State sync is automatic verification of data. Software updates are separately authenticated releases with explicit operator policy, compatibility checks, rolling voting-power budgets and rollback safety. Master is not a remote executable-installation authority.

See plans [12](12-consensus-spec.md), [15](15-network-and-sync-protocol.md), [16](16-genesis-upgrade-and-recovery.md), [19](19-security-and-release-engineering.md), and [32](32-regional-masters-and-public-persistence.md). These are required operating behaviors; runtime deployment, recovery and endpoint selection are not implemented yet.

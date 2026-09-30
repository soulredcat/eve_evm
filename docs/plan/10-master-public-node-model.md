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

## RAM and recovery

A non-voting public node may be ephemeral and rebuild from verified data. Validators may keep hot state in RAM but must durably preserve signing safety, recent blocks and recovery state. Masters keep durable data on storage with a measured fsync/recovery policy.

## Failure table

- Master offline: finality continues only while validator quorum and required data remain available; master catches up later.
- All RPC public nodes offline but validators connected: consensus can continue; user ingress may be unavailable.
- Validator quorum unavailable: new finality stops; master preserves the previous finalized history.
- Corrupt master/snapshot: reject unauthenticated or mismatching data; obtain it from another source.
- Signing-key state lost: validator does not resume blind signing; follow the fencing/recovery runbook.

## Access and updates

Public participation is not bounded by a protocol-wide node count, but peer connections, bandwidth and active consensus membership are bounded independently. Use P2P and relays so every public node need not connect to master.

State sync is automatic verification of data. Software updates are separately authenticated releases with explicit operator policy, compatibility checks, rolling voting-power budgets and rollback safety. Master is not a remote executable-installation authority.

See plans [12](12-consensus-spec.md), [15](15-network-and-sync-protocol.md), [16](16-genesis-upgrade-and-recovery.md), and [19](19-security-and-release-engineering.md).

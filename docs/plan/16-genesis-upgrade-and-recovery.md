# 16 — Genesis, upgrades and recovery

## G01 — Development genesis

Create a deterministic genesis builder with an explicit development flag. The local profile uses EVM chain ID 31337 and consensus network name `eve-local-v1`; this is a local-only convention, not a globally reserved mainnet identifier. Real networks must choose and validate their identity separately.

Genesis contains schema/protocol versions, immutable network identity, consensus parameters, EVM fork target, initial accounts and code, system module definitions, four validator public keys/powers, initial bonded stakes, fee split, resource limits, initial timestamp, upgrade policy and evidence/unbonding parameters. Validate duplicate addresses/keys, overflow, reserved-address conflicts, stake funding and nonempty valid voting power before writing files.

Generate keys and faucet allocations into a task-owned ignored development directory. Never commit private keys. Fixed public test fixtures may use clearly unsafe deterministic test keys only inside isolated test vectors; they must be impossible to select as production configuration. Genesis supply equals the sum of funded balances, including escrows; there is no arbitrary hidden mint.

Canonical genesis hashing uses a specified binary encoding after parsing/validation, not filesystem JSON whitespace. The immutable genesis digest does not recursively include itself or post-genesis app hashes. B0 must freeze encoding and fixtures. Startup refuses a database with another genesis/config identity unless a documented offline migration is invoked.

## G02 — Development-only all-in-one mode

`DEV_ALL_IN_ONE` composes shared execution, a local test producer and storage to implement the early MASTER_ONLY prototype. It must bind only to loopback by default, use development keys and visibly label its status. Production-mode master configuration rejects the local-producer switch and any production consensus key.

One process can host multiple logical development roles, but tests must also run three separated runtime entry points. Co-location cannot become a hidden master RPC dependency in production.

## G03 — Protocol activation

Protocol/fork changes activate at a committed height using an authenticated upgrade record, with code/config digest, old/new version, migration identifier and activation height. The initial devnet may load a schedule from genesis. Dynamic devnet proposals use the owner-configured test governance path and validator consensus; a release manifest alone never changes chain rules.

Unsupported-version nodes stop safely at the activation boundary rather than voting under obsolete semantics. Test mixed-version rollout, pre-activation compatibility, activation and post-activation consistency. Database schema version and network protocol version are distinct; a binary rollback may be unsafe after a one-way migration.

Mainnet governance authority, quorum/timelock, supply and validator admission policy need explicit owner approval and security review. Do not treat an example devnet admin key as an authorized mainnet controller.

## G04 — Software deployment

Signed software metadata describes version, platform, binary digest, source commit, protocol range, schema compatibility and migration requirements. Operators control whether updates are applied. Use staged/rolling rollout with remaining online voting power strictly above the consensus threshold, not merely 'one node at a time'. Preserve durable sign state across restarts.

Maintain a rollback plan for binaries and a separate forward-recovery plan for irreversible schema changes. Never roll back consensus signing state. Source verification, release thresholds and packaging tests are in plan 19.

## G05 — Recovery runbooks to implement and rehearse

### Master crash or corrupt tail

Stop exposing ready status. Open the durable store, verify metadata/checksums and inspect last complete height. Recover database WAL/segment references, replay any decided but unapplied blocks, and compare authenticated commitments. If local corruption cannot be safely repaired, restore into a new namespace from a verified checkpoint and replay. Do not overwrite the only damaged copy before preserving diagnostic evidence.

### Master offline while validators continue

Read last durable execution and authenticated-state heights, fetch missing header/validator history and finalized data from peers, verify, apply and persist. Do not request votes or rewrite validator history. Report lag until caught up.

### Validator crash

Restore consensus WAL, sign-state and application state consistently. Reconcile decided/applied heights through engine-supported replay. Refuse signing if last-sign safety cannot be established. Fence the old signer before activating any replacement; key rotation is a protocol operation, not deleting a file to get past an error.

### Validator quorum lost

Keep serving clearly labelled last-finalized data where possible. Do not lower the threshold, appoint master as emergency leader, or accept a minority fork. Restore enough legitimate validators and retained data. An owner-directed disaster restart with a new genesis is a different network event, never a transparent continuation.

### Public RAM replica lost

Remove readiness, load a trusted checkpoint/snapshot, verify validator history and roots, catch up, then re-enable RPC readiness. Do not return empty balances while rebuilding.

### Region lost or partitioned

Continue only on a partition that satisfies consensus and data requirements. Independent master followers may resynchronize without any consensus election. Shared storage namespaces require fencing to prevent concurrent writers; validator signer failover requires separate stricter fencing.

### Bad upgrade

Stop affected signing safely, identify the last agreed height, preserve evidence, test compatible rollback or forward repair in a copy, and follow the announced activation procedure. Never reset state roots or mark a failed migration complete to restore a green dashboard.

## G06 — Operating watermarks

Expose consensus_finalized_height, execution_applied_height, durable_height, authenticated_snapshot_height, oldest_retained_height and signer_last_height/round/step where safe. Operators must know whether a lag is execution, storage, sync, validation or public indexing. Do not reveal private sign bytes or keys through health endpoints.

## Acceptance

T-G01: same genesis inputs produce the same digest/roots; conflicting genesis refuses startup.
T-G02: invalid allocation, duplicate validators and reserved-address collisions fail before launch.
T-G03: development producer cannot start against production profile.
T-G04: forced crashes before/after each commit boundary recover consistent state.
T-G05: lost signing-state blocks unsafe restart; fenced replacement cannot double-sign.
T-G06: master/region outage and all-public-replica loss recover from retained finalized data.
T-G07: planned upgrade/rollback tests preserve quorum and detect incompatible schema rollback.
T-G08: quorum loss never triggers master-only finality.

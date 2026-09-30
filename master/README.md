# Master runtime

Planning only; the runtime is not implemented yet.

Production role: verify finalized consensus provenance, import authenticated state transitions, maintain durable state/history, publish snapshots, support recovery and report replication lag. The master has no production voting/proposal authority and is not a mandatory transaction hop.

A local development all-in-one harness may compose shared execution and test-validator components. It must not leak that authority into MASTER_SYNC_ONLY.

The master host/distribution may explicitly run master, public and validator components together. Keep their entry points, configuration, credentials and authority separate; composing a validator on the same host does not make the master role a voter. Public and validator distributions must each be independently copyable/buildable/runnable without this private master implementation. Reproducible packaging includes only the required canonical reusable components and never creates a second hand-maintained consensus implementation.

Start with one master and one public runtime alongside the four-validator devnet baseline. One master can serve several public nodes through bounded object distribution or relays. A single-validator harness is local development only.

Introduce `zone_id` as routing/operational metadata from the start, distinct from network name, immutable genesis hash and EVM chain ID. Zones do not grant voting power, state ownership or finality. Evolve toward two mutually synchronized independent finalized-history replicas, then a planned ten masters across regions; this is not deployment, hardware purchase or launch authorization.

Every master verifies validator finality, historical set transitions and commitment binding independently, including imports from another master. Use separate local storage namespaces; mutual sync never merges writable EVM states. Fence writers that share one mutable namespace without requiring a global writable-database lease across independent replicas. Report actual finalized, applied, durable and authenticated-snapshot heights.

Expose logical sync endpoints/relays without public database/admin access or privileged master inventory. Public nodes select eligible sources by authenticated identity/network/profile/proofs and verified lag/data availability before measured service latency and verified-data throughput. Support bounded fallback and resume; no endpoint-anonymity guarantee or mandatory per-transaction master acknowledgement exists.

Implement against shared `FinalityVerifier`, `StateStore`, `BlockStore` and sync interfaces. Read plans [02](../docs/plan/02-state-and-storage.md), [10](../docs/plan/10-master-public-node-model.md), [14](../docs/plan/14-block-and-state-commitment-spec.md), [15](../docs/plan/15-network-and-sync-protocol.md), [16](../docs/plan/16-genesis-upgrade-and-recovery.md), [19](../docs/plan/19-security-and-release-engineering.md), [22](../docs/plan/22-code-layout-and-dependency-policy.md) and [32](../docs/plan/32-regional-masters-and-public-persistence.md) before implementation.

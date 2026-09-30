# Master runtime

Planning only; the runtime is not implemented yet.

Production role: verify finalized consensus provenance, import authenticated state transitions, maintain durable state/history, publish snapshots, support recovery and report replication lag. The master has no production voting/proposal authority and is not a mandatory transaction hop.

A local development all-in-one harness may compose shared execution and test-validator components. It must not leak that authority into MASTER_SYNC_ONLY.

Implement against shared `FinalityVerifier`, `StateStore`, `BlockStore` and sync interfaces. Never expose database/admin access through public RPC. Read plans 02, 10, 14–16, 19 and 22 before implementation.

# 09 — Delivery Roadmap

## Phase 0 — Protocol specification

Deliverables:

- block/header schema;
- transaction envelope;
- state commitment rules;
- fork/version rules;
- finality semantics;
- validator identity/signature rules;
- crash consistency model;
- threat model.

Exit criterion: two independent implementations could read the spec and agree on the same block/state result for test vectors.

## Phase 1 — Single-node deterministic EVM

Deliverables:

- EVM engine integration;
- transaction decoding/signature validation;
- deterministic state transition;
- block builder;
- state root;
- receipt/log generation;
- durable state + WAL;
- restart recovery;
- minimal JSON-RPC.

Exit criterion: deterministic replay produces identical blocks and roots.

## Phase 2 — Public node + validator

Deliverables:

- public RPC node;
- authenticated internal transport;
- validator runtime;
- block/state verification;
- state-delta replication;
- snapshot bootstrap;
- validator catch-up.

Exit criterion: public node can be destroyed, rebuilt from snapshot+deltas, and independently reach the canonical root.

## Phase 3 — Consensus, staking and rewards

Deliverables:

- validator-set lifecycle;
- consensus/finality implementation;
- staking/delegation;
- uptime/work measurement;
- fee accounting;
- 40/30/30 configurable basis-point split;
- penalty/jail/slashing rules.

Exit criterion: deterministic epoch accounting and adversarial consensus tests pass.

## Phase 4 — Parallel execution

Deliverables:

- dependency/conflict model;
- scheduler;
- re-execution path;
- deterministic merge/commit;
- high-contention tests;
- multi-worker benchmark.

Exit criterion: parallel execution produces exactly the same canonical result as reference serial execution.

## Phase 5 — Multi-region

Deliverables:

- regional ingress;
- regional batch format;
- bulk state-delta transport;
- global finality integration;
- failover/fencing;
- cross-region state ownership;
- cross-domain transaction semantics.

Exit criterion: loss of one region does not corrupt canonical state; documented failover succeeds.

## Phase 6 — Scale validation

Deliverables:

- reproducible benchmark harness;
- multi-node deployment automation;
- load generators;
- telemetry;
- capacity model;
- bottleneck reports.

Scale in measured steps:

```text
10k → 50k → 100k → 250k → 500k → 1M aggregate finalized TPS
```

No phase advances solely because an ingress/load generator reports the target request rate.

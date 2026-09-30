# 01 — System Architecture

## Logical roles

### Public Node

Responsibilities:

- public JSON-RPC and WebSocket endpoints;
- transaction ingress and basic stateless validation;
- current-state RAM replica/cache;
- validator participation where configured;
- verification of canonical block/state commitments;
- rate limiting and abuse isolation.

Must not have direct filesystem/database access to master storage.

### Master / Canonical Node

Responsibilities:

- canonical state persistence;
- sequencing/block construction in the initial architecture;
- state-transition coordination;
- snapshot/WAL management;
- state-delta publication;
- protected internal API only.

The logical canonical role may later use active/standby or replicated infrastructure. Physical high availability must not accidentally create two independent canonical histories.

### Execution Workers

Responsibilities:

- execute deterministic EVM workloads;
- operate on declared/scheduled state domains;
- return deterministic execution results;
- expose no public administrative surface.

### Validator

Responsibilities:

- independently validate proposed blocks/state transitions;
- participate in finality according to the selected consensus protocol;
- produce signatures/votes that can be verified by public nodes.

### Archive / Indexer

Responsibilities:

- historical blocks, receipts and logs;
- explorer/search/analytics queries;
- optional long-term state history.

Not part of canonical transaction execution.

## Traffic separation

Keep at least four logical lanes independent:

1. public RPC traffic;
2. consensus/finality traffic;
3. execution/state-delta traffic;
4. administrative/management traffic.

State synchronization must never be able to starve finality traffic merely because both share a large bulk stream.

## Open decisions

- initial consensus algorithm;
- proposer rotation rules;
- canonical master failover semantics;
- execution worker ownership;
- deterministic scheduler design;
- global vs regional block construction;
- finality guarantees when validator availability drops below quorum.

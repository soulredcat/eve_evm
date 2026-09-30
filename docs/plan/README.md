# EVE EVM — Planning Index

This directory defines the work to be completed before and during implementation.

The documents are ordered by dependency. Later plans should not silently override invariants defined earlier; incompatible decisions must update the relevant architecture document explicitly.

## Planning sequence

1. [00-vision-and-non-goals.md](00-vision-and-non-goals.md)
2. [01-system-architecture.md](01-system-architecture.md)
3. [02-state-and-storage.md](02-state-and-storage.md)
4. [03-blocks-execution-and-parallelism.md](03-blocks-execution-and-parallelism.md)
5. [04-master-public-node-security.md](04-master-public-node-security.md)
6. [05-validator-staking-and-economics.md](05-validator-staking-and-economics.md)
7. [06-regional-sync-and-finality.md](06-regional-sync-and-finality.md)
8. [07-developer-compatibility.md](07-developer-compatibility.md)
9. [08-benchmark-and-acceptance.md](08-benchmark-and-acceptance.md)
10. [09-delivery-roadmap.md](09-delivery-roadmap.md)

## Rule for implementation

A component is not considered complete because it compiles. Each milestone must have:

- documented invariants;
- deterministic tests;
- crash/restart tests when stateful;
- adversarial/failure tests where applicable;
- metrics;
- reproducible benchmark commands;
- explicit acceptance criteria.

## Current status

All items are **PLANNED**, not implemented.

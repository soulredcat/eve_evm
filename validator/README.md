# EVE Validator

This directory is reserved for the dedicated EVE Validator runtime.

## Role

The Validator runtime is responsible for production consensus and finality.

Planned responsibilities:

- validator identity and consensus keys;
- staking/registry integration;
- transaction/block execution or deterministic replay;
- block proposal when selected;
- candidate block verification;
- state-root verification;
- voting/signing;
- finality-certificate construction/verification;
- uptime and participation accounting;
- jail/slashing evidence;
- catch-up and rejoin behavior.

## Important boundary

```text
Public Node
= RPC / P2P / transaction ingress

Validator
= execution / proposal / vote / finality

Master
= finalized-state sync / durable persistence / snapshot / archive
```

The Validator must not require direct access to the Master database or filesystem.

## Status

Planning only. Production implementation has not started.

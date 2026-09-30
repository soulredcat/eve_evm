# 00 — Vision and Non-Goals

## Vision

Build an EVM-compatible chain optimized around:

- low-latency regional transaction ingress;
- parallel execution where state dependencies permit it;
- memory-resident hot/current state;
- durable canonical recovery state;
- public validator/RPC nodes separated from protected canonical infrastructure;
- bulk state synchronization across regions;
- protocol-verifiable validator/node rewards;
- developer compatibility with the Ethereum ecosystem.

## Long-term throughput target

The project may target **1,000,000 aggregate transactions per second**, but that number is not a requirement for the first production release and must never be presented without workload definition.

Separate benchmark classes will be required for:

- native/simple value transfers;
- ERC-20-like transfers;
- independent contract writes;
- AMM swaps;
- multi-pool/multi-contract transactions;
- adversarial high-contention state;
- mixed realistic workload.

## Non-goals for the first implementation

- immediate 1M TPS production deployment;
- custom Solidity language;
- breaking EVM behavior merely to improve benchmark numbers;
- storing canonical state only in RAM;
- exposing master/canonical state nodes publicly;
- solving multi-region sharding before a correct single-region chain exists;
- optimizing explorer/analytics databases before core protocol correctness.

## First proof

The first useful proof of architecture is:

```text
signed transaction
→ deterministic EVM execution
→ block construction
→ validator verification
→ durable state commit
→ restart
→ identical canonical state
```

Anything beyond this depends on this path being correct.

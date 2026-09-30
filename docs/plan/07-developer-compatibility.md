# 07 — Developer Compatibility

## Objective

Internal storage and execution architecture may be highly customized, but application developers should receive a predictable EVM-compatible surface.

## Developer-facing targets

- Solidity contracts;
- standard EVM bytecode semantics for the chosen fork/version;
- Ethereum-style addresses and signatures where applicable;
- JSON-RPC compatibility;
- transaction receipts/logs;
- ABI/event compatibility;
- support for common libraries such as ethers/viem/web3 where protocol compatibility permits;
- wallet integration through standard chain configuration.

## Initial RPC surface

Prioritize a useful compatibility subset:

- `eth_chainId`;
- `eth_blockNumber`;
- `eth_getBalance`;
- `eth_getTransactionCount`;
- `eth_call`;
- `eth_estimateGas`;
- `eth_sendRawTransaction`;
- `eth_getTransactionByHash`;
- `eth_getTransactionReceipt`;
- `eth_getBlockByNumber`;
- `eth_getLogs`;
- subscription/WebSocket methods after core correctness.

## Compatibility policy

Every release must declare:

- EVM hard-fork target;
- supported precompiles;
- gas schedule;
- RPC differences;
- unsupported methods;
- chain-specific extensions.

Do not silently change EVM semantics for performance.

## Native extensions

Optimized native features may be added later, but they should be additive whenever possible.

Examples to research:

- explicit access/read-write hints;
- fast native pool primitives;
- batch transaction extensions;
- parallel-execution hints.

A standard Solidity developer should not need to understand the internal WAL, state database, snapshot, or regional replication architecture.

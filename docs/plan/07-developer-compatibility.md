# 07 — Developer compatibility

Developers interact through Solidity bytecode, ABI, Ethereum-style signed transactions and JSON-RPC. They should not need to know the master database, WAL format or replica topology.

The first executable compatibility target is explicitly **Shanghai EVM semantics**, with protected legacy transactions and typed transactions 1 and 2. This is a bounded development baseline, not a claim to implement the newest Ethereum fork. Compiler fixtures must set `evmVersion` accordingly. Cancun/Prague and later features require a versioned implementation and tests, not silent acceptance.

Preserve ordinary account/storage semantics, synchronous calls, reverts, creation, logs and gas behavior for the declared baseline, except clearly listed chain-specific environment/economic rules. Custom reward distribution, BFT finality, system calls, empty Ethereum withdrawal fields and any unsupported RPC methods must be documented.

Public binaries must support real deploy/transfer/call/receipt/log workflows through at least one TypeScript client fixture. Test a native transfer, ERC-20, minimal AMM, atomic multi-pool operation, revert and contract creation. Client versions and Solidity compiler must be pinned.

Public nodes must return explicit unsupported/pruned/syncing errors instead of plausible fabricated data. A successful raw-transaction response is a transaction hash, not evidence of finality. Expose readiness and finalized height separately from process liveness.

Detailed RPC and developer acceptance: [18](18-rpc-mempool-and-developer-experience.md). Exact execution/transaction policy: [13](13-transaction-and-gas-spec.md). Block and proof mapping: [14](14-block-and-state-commitment-spec.md).

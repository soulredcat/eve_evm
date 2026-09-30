# 03 — Blocks, execution and parallelism

## Block lifecycle

The selected validator proposer constructs an ordered candidate from available transactions. Validators validate the proposal and its execution under the selected consensus adapter. Quorum commits the ordered block; the application persists deterministic results. Master followers import finalized data afterwards.

A transaction can be admitted, proposed, consensus-committed, locally applied/durable, and remotely persisted at different times. These statuses must not be collapsed into one RPC success response. Consensus header hashes, EVM-facing block hashes and post-state commitments are separate typed objects; see [12](12-consensus-spec.md) and [14](14-block-and-state-commitment-spec.md).

## Reference execution

Implement a serial reference first. Given identical parent state, block environment and ordered transactions, every correct implementation must produce identical balances, nonces, storage, gas, logs, receipts and commitments. A reverting EVM transaction may still be valid and consume gas. An invalid transaction envelope must not be silently converted to an included successful transaction.

## Parallel path

An optimistic scheduler may execute transactions against versioned overlays, detect read/write conflicts, and deterministically re-execute before ordered commit. It must track accounts, balances, nonce, bytecode, storage, creation/deletion, logs, refunds and system calls, not merely pool IDs. Access lists are hints, not complete declarations of all dynamic EVM accesses.

Different pool addresses or signatures do not prove independence. Shared token balances, allowance contracts, routers and atomic multi-pool calls create dependencies. A transaction that touches several pools commits all allowed effects or reverts its transaction effects according to EVM rules.

Do not distribute conflicting global-state writes to independent regional masters and merge roots later. No state merge is valid without a defined ordering/conflict protocol.

Required evidence: differential serial/parallel fixtures, hot-pool workloads, nonce conflicts, contract creation, reentrancy and failures under shuffled worker scheduling. Plans [20](20-test-vectors-and-acceptance.md) and [21](21-capacity-and-regional-scaling.md) define the gates.

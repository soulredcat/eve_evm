# Validator runtime

Planning only; the runtime is not implemented yet.

Owns transaction execution/replay, proposal validation, consensus participation, finality and validator lifecycle integration. Uses shared EVM/protocol/state modules and a reviewed BFT adapter; no private master database dependency.

Mandatory durable data: consensus WAL, anti-double-sign height/round/step and sign-byte history, key-fencing state, recent finalized block data and recoverable application state. Hot RAM does not replace these records.

A four-validator equal-power devnet needs three votes for a commit under the selected more-than-two-thirds rule. Loss of quorum stops finality; master never takes over. More validators do not automatically increase TPS because replicas verify the same ordered workload.

Read plans 12–17 and 20–22. A co-located public RPC process does not receive consensus keys. Never clone live signing keys into an active standby.

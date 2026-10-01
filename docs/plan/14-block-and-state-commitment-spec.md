<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# 14 — Blocks, commitments, serialization and persistent state

## S01 — Separate objects

Use distinct types for `ConsensusBlockId`, `ExecutionBlockHash`, `EvmStateRoot`, `SystemStateRoot`, `ApplicationCommitment`, `SnapshotId` and `FinalityProof`. A random 32-byte value is not interchangeable with any of them.

A finalized record identifies network/genesis, consensus height/hash and certificate, execution header/hash, ordered signed user transactions, receipts, state roots, protocol version and configuration digest. Certificate bytes are transported separately; including them in the block they sign would create circular hashing or unstable identity.

## S02 — Ethereum-facing execution header

Use a deterministic Shanghai-shaped Ethereum RLP execution header for EVE's RPC block hash. Encode the standard field order using maintained types: parentHash, sha3Uncles, beneficiary, stateRoot, transactionsRoot, receiptsRoot, logsBloom, difficulty, number, gasLimit, gasUsed, timestamp, extraData, mixHash, nonce, baseFeePerGas, withdrawalsRoot.

No uncles or Ethereum consensus withdrawals are processed by this devnet. Use the upstream canonical empty structures/roots and test vectors, not guessed constants. Difficulty is zero; nonce is eight zero bytes. The beneficiary/environment mapping is in plan 13. `extraData` is `u32_be(protocol_version)` followed by the first 28 bytes of the genesis/config domain digest. The consensus header and its hash remain distinct and follow the external BFT engine.

Transaction and receipt roots use ordered Ethereum trie encodings, including typed envelopes where applicable. Do not hash a JSON array as an Ethereum transaction root. `gasUsed`, bloom and receipts match execution exactly. EVM `BLOCKHASH` returns EVE execution hashes, not consensus hashes.

## S03 — State and system commitments

Use Ethereum-compatible account/storage Merkle Patricia tries for the first EVM state commitment. Preserve nonce, balance, storageRoot and codeHash encoding; contract storage keys/values and absent/zero slots follow the chosen fork. Bytecode is content-addressed and validated against codeHash. Physical storage may change without changing these logical roots.

Use a separate authenticated system trie for validator registry, fee/reward liabilities, parameters, work tasks, evidence and upgrade state. A system key is `keccak256(rlp([namespace_bytes, logical_key_bytes]))`; values use versioned canonical RLP tuples defined in generated schema fixtures. Namespace/key schemas are fixed in B0, covered by vectors and changed only through a migration. No nondeterministic map serialization is allowed.

Define:

```text
ApplicationCommitment(H) = keccak256(rlp([
  "EVE_APP_V1", genesis_hash, protocol_version, H,
  evm_state_root_H, system_state_root_H, execution_block_hash_H
]))
```

The genesis hash binds the immutable chain specification, not a structure containing this same commitment. The execution header must not include its own application commitment. The EVM state root remains usable for ordinary account/storage proofs; a separate proof binds it to the application commitment and certified consensus history.

[Source: Ethereum trie structure](https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/). EVE's system trie and combined commitment are project design choices, not Ethereum protocol fields.

## S04 — Authentication path

Follow plan 12's ABCI height mapping. A snapshot for post-state H is authorized by a verified application commitment, normally carried by the certified consensus header at H+1, or by replay from a trusted state. Verify validator-set transitions and trust freshness. A valid signature on an unrelated hash or height does not authenticate the snapshot.

A state delta contains ordered key operations for both domains, expected base roots, target roots, height range and the relevant finalized block references. Apply it to a staging view only after provenance checks; compute roots and compare. A matching root alone is not an execution-validity proof. Public archival replay uses the ordered transactions as the correctness check.

## S05 — Byte formats and limits

Use canonical RLP for consensus-bound EVE records and Ethereum objects; reuse the engine's own encoding for its consensus objects. Transport envelopes may use a versioned protobuf schema, but transport bytes are not silently substituted for signed/committed canonical bytes.

All integers are unsigned, minimally encoded canonical RLP unless the upstream engine mandates another exact form. Define maximum widths and overflow behavior. Booleans, optional fields, empty data, deletion markers and namespace tags receive explicit encoding vectors in B0. Sort system entries by canonical key bytes for exports; reject duplicates and noncanonical variants.

Version snapshots, deltas, database schema and protocol separately. Hash uncompressed canonical content for identity. Compression and chunking may vary without changing the logical state. Add transport checksums for corruption detection; these do not replace cryptographic provenance.

## S06 — Atomic persistence

An atomic RAM-applied view and a durable storage commit are distinct publications. Default public execution/query working state is RAM-first, with local durable finalized blocks and recoverable checkpoints. Reserve bounded handoff capacity before RAM application. An isolated asynchronous storage worker consumes immutable finalized batches in canonical height order; one durable commit owner controls each local namespace. Each batch binds network, parent/target heights, block identity, the selected storage payload and commitment metadata through B0's compile-tested schema. Execution/query state locks are never held across disk I/O or fsync.

Baseline full `StateStore` durable storage uses one logical RocksDB transaction/write batch boundary for EVM changes, system changes, roots, receipts/indices and durable execution-height metadata. Large immutable block segments may be stored separately only with an ordered protocol:

1. Write the segment payload and checksum; ensure required durability.
2. Atomically commit state and references plus the durable height marker using the database WAL/sync policy.
3. Publish the new durable storage view/height and acknowledgment only after the required sync succeeds.

A crash before the durable boundary may leave an orphan segment, but no partially committed durable height may be advertised. After an acknowledged durable commit, recovery must restore its complete references. A batch in RAM or the OS page cache is not durable. Reconcile a lost queued tail after the last complete local commit from authenticated durable peers and replay finalized blocks through the deterministic transition; reward/evidence IDs prevent duplicate effects. Never prune the only recoverable copy of finalized data.

An explicit public recovery-store profile may store finalized block payloads, authentication, protocol/configuration, security-profile and validator-set history with periodic consistent checkpoints rather than full hot-state changes at every height. Publish its recovery `durable_height` only after required payloads, references and metadata are atomically synced and a complete recoverable replay sequence exists from a verified checkpoint through that height. Report `checkpoint_height` and full state-store durable height separately where applicable. Recovery replays and verifies exact roots/receipts; this profile does not relax the full `StateStore` atomic commit contract above or treat an incomplete replay sequence as durable.

Bound queued bytes, batch count and oldest-item age, including retained immutable buffers. Budget worker CPU, database/page cache, snapshots and compaction independently of execution/RPC work. B0 freezes versioned measured limits; saturation tests exercise those limits and slow/failed sync. Apply backpressure and remove readiness when the declared lag policy is exceeded. Isolation reduces contention but does not guarantee zero CPU, memory-bandwidth or I/O overhead. See [plan 32](32-regional-masters-and-public-persistence.md).

Validator anti-double-sign persistence before releasing a signature remains a synchronous safety obligation. The public worker model does not defer signer durability or weaken the selected consensus engine's commit/recovery contract. Master NVMe followers remain outside the mandatory transaction path.

Do not invent a second general-purpose WAL unless tests prove why the database WAL and consensus block log are insufficient. [RocksDB WAL reference](https://github.com/facebook/rocksdb/wiki/Write-Ahead-Log-%28WAL%29). Pin actual options and verify their power-loss behavior on the target platform.

## S07 — Read isolation and pruning

RPC reads capture one locally verified, finalized and applied height/root. Within an explicit readiness/lag policy, public RPC may serve that immutable RAM view before local durable storage catches up; expose the separate watermarks rather than claiming the unflushed tail durable. An `eth_call` must not read half of one block and half of another. Durable checkpoint export captures one consistent finalized state view, persists its complete checkpoint content and excludes speculative overlays. A full state-store export uses a consistent durable database view. Publish a usable checkpoint manifest only after every referenced chunk meets the required durability policy and S04 authentication is verified.

Retain recent consensus data, validator-set history, signing safety and recovery material independently of archive pruning. Keep explicit watermarks for finalized, applied, durable, authenticated state, checkpoint, snapshot-authenticated and prunable heights. Do not equate finalized H with an authenticated post-state H: S04 and plan 12 retain the H/H+1 binding and replay alternative. Deletion must never cross the recovery/availability limits in plan 15. Database compaction cannot bypass these logical limits.

## Acceptance

T-S01: known empty/nonempty account, storage, transaction and receipt root vectors.
T-S02: execution header hash recomputes from RPC fields; block/hash domains cannot be mixed.
T-S03: system encoding stable across insertion orders/platforms; malformed encodings rejected.
T-S04: delta replay and full transaction replay converge to both roots and the combined commitment.
T-S05: kill at every persistence boundary; recover a complete height, never partial balances or duplicated payouts.
T-S06: concurrent RPC/snapshot export sees one consistent version.
T-S07: prune/compaction/restart retains advertised history and returns explicit errors for genuinely pruned data.
T-S08: a mutated system reward ledger fails the application commitment even when the EVM root is unchanged.

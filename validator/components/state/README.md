# Validator-owned complete logical state

Canonical owner: `validator/components/state/`. This component owns complete
logical account/storage/code/system/history data, immutable views, ordered
journals, common commitment construction and canonical recovery content.
Execution adapters live in `validator/components/execution/src/state/`.
Private database handles and durable commit/cache orchestration belong to
`public/components/recovery-store/`; runtime roles keep their own authority.
No database, live networking, clock, signer or master implementation is used here.

Read plans [14](../../../docs/plan/14-block-and-state-commitment-spec.md),
[16](../../../docs/plan/16-genesis-upgrade-and-recovery.md),
[22](../../../docs/plan/22-code-layout-and-dependency-policy.md),
[23](../../../docs/plan/23-task-backlog-and-execution.md) and
[25](../../../docs/plan/25-folder-hierarchy-and-file-function-policy.md) before edits.

## Data and view semantics

`StateIdentity` binds genesis, network name, EVM chain, supported protocol/profile,
key epoch and configuration digest. The initial configuration digest equals the
immutable canonical genesis digest. Protocol 1/CLASSICAL_DEV is supported; an
unsupported identity or activated hybrid profile fails closed.

`CompleteState` materializes all accounts, nonzero slots, content-addressed raw
bytecode, canonical system records and retained execution hashes in ordered maps.
Explicit empty-account presence differs from absence. Zero storage is omitted;
missing storage in this complete materialization is known zero. Nonempty account
code must have a referenced blob whose Keccak matches codeHash. Empty code is
known by the maintained canonical empty-code hash, without requiring a blob.
Missing referenced code/history returns an error and never invokes a remote source.

`StateVersion` binds identity, height/timestamp, execution hash, EVM/system roots,
optional app commitment and local complete-content digest. App commitment is
absent only at height 0 because the frozen EVE_APP_V1 post-state encoder starts
at positive heights. Complete-content digest includes unused code blobs and hash
history outside the EVM/system tries. It prevents reuse of one locally trusted
version with altered auxiliary data; it is not a finality/PQ-security enhancement.

`capture_state_view` consumes complete data, matches its version and retains an
immutable Arc. Each reader sees one version. Read accessors expose account/code/
storage/system/history without mutation or I/O. A requested EVM BLOCKHASH inside
the previous-256 window requires data; current/future/outside-window requests are
explicitly absent. RPC/public readiness and actual verified-finality factories
are runtime/B3/B4 work. These public data types and locally consistent views are
not authenticated capabilities and do not establish production finality.

## Genesis and canonical commitments

`initialize_development_state` validates the development genesis, debits each
self-bond from its funded owner and credits aggregate staking custody at f100
once. Funded supply is conserved; f101/f102 are absent until nonzero fee credit,
not explicit empty pool accounts. User-declared account/code presence is retained.
The system trie initializes owner-keyed validator records, the `fee/pool` singleton
and named protocol/economics parameter records. Initial self-bond details remain
bound by immutable genesis; B5 lifecycle/liability rules are not implemented here.
The factory produces a real Shanghai-shaped execution header/hash and history
entry at height 0; it produces no consensus certificate or voter fallback.

The sole handwritten EVM root builder uses maintained Alloy account/storage MPT
primitives. The original execution `compute_state_root` API projects its complete
REVM oracle data and delegates here. A partial lazy cache is never a world-state
root. System MPT leaves use existing protocol-config namespace/key/value encodings,
with sorted canonical trie keys. Header/app encodings reuse the frozen component.

`build_state_commit` derives only the candidate target's own execution-history
entry from its supplied header, then builds/validates the version. This permits
constructing alternate same-height candidate data for replay rejection; it cannot
rewrite a trusted old view. Direct `validate_state_version` compares the actual
current hash/content and never normalizes source data. Past/future history rules
remain strict. `validate_block_hash_history` compares every supplied past entry
to caller-selected retained header hashes and requires the previous-256 window.
The storage owner supplies actual retained headers; this helper authenticates none.

## Atomic candidate journals

`StateJournal` binds exact parent version and next height. Operations remain in
explicit execution order: PutAccount preserves slots; DeleteAccount removes its
storage; explicit ClearStorage and delete/recreate reset it. Storage zero/delete
removes a slot. Code insertion validates its hash; deleting code still referenced
after the complete journal fails. System insertion validates canonical key/value
binding. Existing past execution hashes cannot be overwritten or removed.

Apply validates the parent, preflights operation/count byte bounds, and mutates
only a private candidate. Any failure discards all changes. Projection produces
deterministic differences, preserves untouched slots and bounds each incremental
operation before retaining it. No `CacheDB::nest().flatten()` is used: replacing
a partially loaded outer DbAccount would lose untouched inner storage.
Journal encoding is `EVE_STATE_JOURNAL_V1`, parent version, target height and ordered
tagged operations. The byte budget uses a conservative bounded preflight before
serialization; it may reject a tight local cap without redefining consensus validity.

## Recovery format and validation

The canonical uncompressed commit is RLP:

```text
[EVE_STATE_COMMIT_V1, optional_parent_version, target_version,
 [identity, accounts, codes, system_records, execution_history],
 [Shanghai_header, ordered_signed_transaction_bytes, ordered_receipt_bytes]]
```

Accounts encode address, nonce, balance, codeHash and ordered nonzero slot/value
pairs. Code/system/history maps encode sorted key/value pairs. Versions encode
identity, height, timestamp, execution/EVM/system hashes, optional app commitment,
then complete-content digest. Optional values use `[0]` / `[1,value]`.
Integers use maintained minimal RLP. Decoder bounds lengths/counts before allocating,
rejects duplicates/trailing bytes, and reencodes the validated object to reject
noncanonical ordering/representations. System decoding reuses the existing encoder
for exact-byte validation. `compute_commit_identity` hashes whole canonical bytes;
the repository also retains/compares those exact bytes for replay identity.

Commit validation checks the complete roots/content/version, exact parent/height,
nondecreasing agreed time, genesis/protocol header extraData, Shanghai field set,
30M gas limit, transaction/receipt trie roots, canonical protected type 0/1/2
envelopes, matching receipt types/status, cumulative gas and actual logs bloom.
Receipt topics/bytes and aggregate recovery payload are bounded before use.
These are structural/canonical checks, not nonce/balance execution, source finality
or certificate verification. Only the execution path validates ordered transitions;
matching a peer-supplied root is insufficient. Durable sync/ack/error fencing,
checkpoint publication and cached read service are the storage/runtime owner's work.

## Complete-state reference execution

`to_revm_state` materializes the full validated EVM state and required hash window
into the existing EmptyDB-backed oracle. `execute_complete_state` runs the same
Shanghai executor, converts the complete result, updates the system fee/burn ledger
once from its existing FeeAllocation and returns state plus a checked journal.
It does not split fees twice or credit escrows a second time. Native B5 handlers,
actual consensus and durable signing remain separate work.

The caller must supply an actually reserved clone budget. A deterministic
conservative charge includes both materialized oracle caches, code analysis and
conversion scaffolding; insufficient reservation rejects before construction.
Original retained views, raw transactions/receipts and other runtime allocations
need separate accounting. This is not an allocator/RSS cap or zero-overhead claim.
Large states cannot be passed as a cheap small overlay. Optimized sparse/persistent
views require later equivalence/resource evidence; correctness is preserved first.

## Verification

Component tests exercise funded genesis, code/hash failures, immutable reads,
system-only commitment changes, codec roundtrip/noncanonical rejection, slot
preservation, zero/delete/recreate and failed journal atomicity. Existing eight EVM
reference cases remain unchanged. Independent EthereumJS literal roots and real
execution/storage/crash integration belong to `tests/acceptance/state-recovery/`.
Process crashes are not hardware power-loss proof. Run strict lint, format,
structure and the full B1 gate before integration; no acceptance follows from a
component build alone. Role copies must still package this canonical source and
pass unrelated-directory build/run; no private master dependency is introduced.

# State and recovery acceptance

Canonical owner: shared B1 correctness/security acceptance. This test package
exercises validator-owned state/execution/protocol contracts and public-owned
persistent storage through their narrow interfaces. It owns no production state,
database handle, signing authority, master finality or custody implementation.

Tests compare literal independently derived trie roots, exact state and receipt
bytes, fee/supply identities, complete commit markers, replay and captured views.
Independent root fixtures identify their source tool/version and derivation;
expected roots are never assigned from unchecked outputs of the code under test.

Process tests terminate only development children created by their test. Actual
process interruption is not hardware power-loss evidence. Missing tools or
coverage fails instead of silently skipping. Generated keys, databases, raw logs
and oracle dependencies remain inside ignored task-owned local storage.

Development harness tests must reject production identity/authority and preserve
the distinction between locally produced data, durable state and validator
finality. B1 acceptance does not close distributed consensus, signer safety,
authenticated sync, public worker, bridge or secured throughput gates.

Read plans 14/16/22/23 and the canonical component READMEs before changing a case.
Run the integrator's complete registered B1 gate; individual tests do not prove
that every mandatory bulk requirement has passed.

## Concrete coverage

| Family | Cases and practical scope |
|---|---|
| T-S01 | Independent EthereumJS account/storage/code/system roots, zero/delete/recreate, missing/corrupt code, complete-versus-absent views and real genesis allocations |
| T-S02 | Literal EIP-155 transaction and receipt bytes, ordered roots, malformed/trailing/type/chain rejection, header/marker/gas/bloom binding |
| T-S03 | Canonical system-key/namespace and insertion-order invariance; invalid storage representation fails |
| T-S04 | Real signed Shanghai execution equals journal replay and literal post-state; untouched slots and exact 40/30/30 fees survive; invalid/stale/budgeted work leaves the parent intact |
| T-S05 | Whole synced commit/marker/replay, exact offline corruption rejection, actual task-child termination before/racing/after commit, staged export exhaustion and incomplete activation rejection |
| T-S06 | Captured DB/RAM views remain one version; exports restore only their captured version into a new namespace |
| T-S08 | A validly encoded but mutated fee ledger changes system/application/content commitments while EVM/header hashes remain unchanged |
| T-G01/T-G02 | Real validated development genesis preserves source-order identity, funded supply and single escrow debit/credit; bad allocations/network/profile reject |
| T-G03 | Actual master CLI rejects production/master-sync-only/unacknowledged authority and private material, and reports local durability with authenticated finality false |

The process race checks complete old-or-new outcomes around an actual storage
call; it does not identify an internal fsync syscall boundary. Export interruption
uses the real declared byte limit, with synced partial frames and no ready
manifest. Other snapshot corruption cases reconstruct missing/truncated inputs;
they are not observed hardware crashes. Safe pruning, signer durability,
authenticated network recovery and public persistence-worker interference are
later requirements, not passing results supplied by this package.

Run `cargo test --locked -p eve-state-recovery-acceptance --all-targets`. The
master development binary must be built from the same source first, or supplied
through `EVE_MASTER_DEV_BINARY`. Missing binaries fail. All test data lives under
ignored root `local-tests/b1-acceptance/`; every child is waited/killed by the test
that created it. The static reference fixtures require no npm/network operation
when running the acceptance suite.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# B4 local implementation and verification

B4 remains IN_PROGRESS. Its complete gate is NOT_IMPLEMENTED; no hosted B4,
master-offline recovery, default public capacity or full bulk acceptance is claimed.
Local-first publication batches remain required. Accepted B3 main is `c41ba2f`.

## Compact replay and public RAM checkpoint

Local commit `d9c0c0fe84b3bdb3eb98ad5ac8e660dd0d25e11d` has tree
`839345fedecd9e657a52557121de736bd59cf20f`; it is unpublished. It follows local
foundation `c6376c5` and the accepted-B3 merge `463975b`.

It implements canonical block/recovery codecs, actual sealed transaction replay,
prospective opaque cursors, nonblocking synced acknowledgements and a public-owned
immutable applied-state service. The public service admits empty blocks only.
Captured views and unacknowledged tails retain their logical resource charges.

| Verified scope | Result |
|---|---|
| State/storage packages | 81 cases passed |
| Executor package | 28 cases passed |
| Finality verifier | 47 cases passed |
| Public package | Full 38-case packet, then 14 applied cases; union covers all 39 discovered cases |
| Format / strict scoped Clippy / release | Passed; local release build 49.59 seconds |
| Structure / ownership | 1,819 / 1,853 files; zero violations |

All case results above have zero failures or ignored cases. Independent review
cleared the index and outgoing local history. Only README.md Markdown is tracked;
raw outputs stay in ignored local-tests/b4-preparation. Build timing is not TPS.

Review repaired queue-count accounting, shutdown tail charge transfer, final exact
Duration age checks and decoded-row/read-lease lifetime. The Drop adapter now
matches its exact reviewed standard-trait registration. This service uses estimated
logical accounting, without allocator/RSS, physical disk-stall or power-loss proof.

## Authenticated import foundation

The subsequent local source adds bounded journal decoding and a private authenticated
import capability distinct from independent replay. Complete borrowed preflight
precedes operation-vector/code/system allocation. It preserves all ten operation
tags and repeated/delete-recreate order, with a fixed-depth system-record scanner.
Aggregate decode limits are operator policy; fitting final state does not imply
every journal fits those same decode limits.

Candidate reservation counts transient insertions even when later deleted. Parent
identity is checked before encoding/measurement. Imported roots and execution hash
must match certified H+1; native H/H+1 data, fixed enrolled owners, execution header
context, receipt roots and exact current execution-history entry are checked.
Fees are contained in the delta and are not applied again.

| Verified scope | Result |
|---|---|
| State | Full 42-case packet, then four reservation cases after repair; 43 distinct cases covered |
| Journal codec / reservation | 18 / four new cases passed |
| Finality verifier | 68 cases passed, including 21 new import cases |
| Capability privacy | Six cases rerun after sharing one compiler fixture; passed |
| Public applied regression | 14 cases passed |
| Format / strict five-package Clippy / release | Passed; local release build 24.40 seconds |

These are local checks on the new source, with zero failed/ignored cases. Full B4
source gates and revision-bound checkpoint review remain separate obligations.
See [import contract](../../../validator/components/finality-verifier/src/recovery/import/README.md)
and [public applied contract](../../../public/src/sync/applied/README.md).

The initial import contract requires the exact local parent, including auxiliary
digest. A matching-root snapshot with another auxiliary representation may refuse.
Unused hash-checked code and reconstructed content_digest are local representation,
not independently certified fields. An explicit outside-assumption fixture certifies
a nonce-invalid outcome: import accepts that certified outcome while real replay
rejects execution. This is the documented mode distinction, not stronger security.

## Required continuation

Implement bounded import wire/storage/public admission, authenticated snapshots,
fragmented large records and complete logical durable markers, live public/master
followers, retained peer-tail retrieval and master-offline recovery. Preserve all
T-N01–T-N07, T-N09/T-N10, T-S04–T-S08 and T-G04/T-G06 gates. No valid maximum payload
may be made to fit by silently raising limits or omitting data.

Fixed-genesis classical assumptions remain explicit. Dynamic authority, secure
profile acceptance, standalone copies, physical resource/fault evidence and 1M
aggregate finalized TPS remain unfinished. Continue existing bulks in dependency
order; this document does not authorize mainnet, real funds or external programs.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# B4 local implementation and verification

The latest [follower foundation evidence](follower-foundation/README.md) records actual local public CLI recovery, master archive and authenticated snapshot prerequisites. B4 remains IN_PROGRESS. Its complete gate is NOT_IMPLEMENTED; no hosted B4,
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

## Compact wire and explicit public modes

The next local source integrates EVE_IMPORT_V1 with sealed borrowed preflight over
one immutable input and frozen budget. Exact aggregate sizing precedes component
output; decoding compares complete canonical reencoding against those same bytes.
The compact limit remains 4,198,312 bytes. State-owned typed/wire candidate estimates
share one cost calculation and count transient/repeated writes before decoding.

Public application has explicit EmptyReplay and AuthenticatedImport modes. The
legacy constructor/domain remain unchanged. Import uses a separate SHA-256 storage
domain, actual-genesis sizing and actual wire counts; it does not charge REVM maxima.
Wrong-mode opening rejects even at genesis. Captured publications expose the mode
and retain charged private state; synced-prefix/tail/age behavior remains shared.

| Verified scope | Result |
|---|---|
| State | 61 cases passed, including 15 measurement/wire-sizing and three genesis cases |
| Finality verifier | 85 cases passed, including 17 new wire cases |
| Public | 48 cases passed, including nine new imported application cases |
| Format / strict five-package Clippy / release | Passed; local release build 26.79 seconds |

All executed cases above have zero failures or ignored cases. Actual signed
nonempty import is tested with paused append, two admissions, RAM capture, exact
reopen and fee equality. The pause is a functional fixture, not hardware stall or
power-loss proof. Independent production review found no remaining blocker in
mode, wire binding, estimate/lease ordering or failure retention. Build time is
not throughput. Full source gates/checkpoint review remain separate obligations.
## Segmented persistence foundation — 2026-10-04

An explicit part-buffer policy v2 preserves numerical 8 MiB encoded parts,
32 MiB queue bytes and four retained parts. It does not rename v1 batches.
The derived classical logical bound is 21,025,569 bytes; six physical segments,
three data parts and one marker part retain at most 21,027,288 encoded bytes.
Physical opaque records keep their original 4,198,400-byte record/read ceiling.
Every actual database transaction writes one physical record.

Canonical segment records use 177-byte headers and 32-byte hashes. Marker size
is 225 bytes plus 40 per reference, at most 465 bytes. Checksums/local anchors
establish local integrity, never finality. A complete logical acknowledgement is
emitted only after all actual segment syncs and the marker sync match the planned
cursor chain. Physical sequence is distinct from execution height.

Atomic part reservations include encoded capacities, count slots and metadata.
Cancelled tickets, failed writes and worker panics retain full owned tails.
A pool-owned worker lifetime lease fences duplicate scratch reservations across
repository paths and remains active through retained tickets and detached threads.
Startup validates actual marker membership; tighter segment and logical-lag
limits reject before admission. No publication lock spans repository I/O.

| Verified scope | Result |
|---|---|
| Node policy | Full 26 cases passed; ten new segmented cases rerun after lint correction |
| Segment codec | 16 cases passed |
| Segmented worker | 19 cases passed after startup, cap, lag and worker-fence repairs |
| Finality verifier | 99 cases passed, including 14 logical V2 transport cases |
| Strict four-package Clippy / release | Passed; local release build 21.12 seconds |

All executed cases above have zero failures or ignored cases. Full logical V2
transport keeps every original component/transaction/header/consensus bound;
its larger envelope is explicit. Decoder input and budgets remain sealed together.
The large transport fixture claims no certificate or execution validity.
Privacy compilation now selects an actually compatible verifier/state rlib pair
with a successful dependency probe before checking the required compiler error;
an unrelated duplicate-crate error cannot satisfy a privacy test.

At this historical checkpoint, these foundations were not yet connected to complete logical-prefix replay or public applied publication. Orphan reconciliation, authenticated reconstruction,
snapshots, live follower entry points, peer-tail recovery and full B4 gates remain
mandatory. Source/publication checks and a coherent local checkpoint follow these
scoped results. Build timing is not throughput or physical fault evidence.
## Required continuation

Finish authenticated snapshot transfer, resume and atomic activation, master-offline
missing-tail recovery, retention/last-copy policy, independently observed readiness
and resource/fault acceptance. The linked scoped follower foundation is implemented
locally. Preserve all
T-N01–T-N07, T-N09/T-N10, T-S04–T-S08 and T-G04/T-G06 gates. No valid maximum payload
may be made to fit by silently raising limits or omitting data.

Fixed-genesis classical assumptions remain explicit. Dynamic authority, secure
profile acceptance, standalone copies, physical resource/fault evidence and 1M
aggregate finalized TPS remain unfinished. Continue existing bulks in dependency
order; this document does not authorize mainnet, real funds or external programs.

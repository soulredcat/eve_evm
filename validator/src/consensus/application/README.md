<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator application lifecycle

Canonical owner: the private validator runtime's actual ABCI application. This
domain composes the sole canonical state/EVM executor and public-owned storage
contracts. Native consensus ordering, quorum and locks remain in pinned CometBFT.
Application durability supplies recovery material and never grants finality.

Root bootstrap provides validated development genesis, native parameters and
validator/proposer-owner mapping. `open_application` validates canonical identity,
the activation-one genesis registry, native block/gas/key bounds and the shared
signer/state service, bootstraps the derived history index, and reconciles replay
metadata before returning. Production launch and unsupported profiles remain
root bootstrap failures; no master or release authority decides a block here.

| Operation | Responsibility |
|---|---|
| `application_info` | Return actual durable application height/hash; H0 uses the frozen development content digest, positive heights use canonical application commitments |
| `initialize_application` | Match native height one, time, chain, parameters, keys/powers and the exact semantic public specification against trusted root input |
| `check_transaction` | Canonical decode and cache-first local account admission without changing nonce/balances or establishing inclusion |
| `prepare_proposal` | Conservative nonce/upfront/gas/protobuf-byte selection, followed by actual isolated execution and at most one valid-prefix re-execution |
| `process_proposal` | Execute the actual supplied sequence through the sealed approval factory, independently of local mempool contents; publish only a current complete-context approval |
| `finalize_block` | Execute/reuse the exact native-decided candidate, preserve bounded replay input before state sync, and return canonical receipts/gas/app hash |
| `commit_application` | Sync complete state, compare actual acknowledgment, then sync completion metadata and retire old-height approvals |

Private application-channel callbacks require the real accepted Unix peer's
live-process/image/credential guard. Process/Finalize consume root-issued immutable
tokens with separate source tags; a finalized callback is not fresh unlocked-round
evidence. Native ProcessProposal does not include a full header/part-set body.
The trusted pinned local engine supplies its validated hash/data relationship;
this domain does not synthesize missing header fields or turn a local peer token
into a portable quorum proof. Network followers require independent certificates,
app-height anchoring and actual replay under their applicable trust rules.

Proposer owner and previous consensus hash come from trusted configuration and
retained replay records. Only genesis has a zero preceding consensus hash. Empty
blocks still preserve their actual current native hash for the next environment.
Execution performs no network, clock, master or external program calls. A valid
EVM revert remains ABCI code zero with its canonical status-zero receipt and gas;
an invalid envelope/ordered transition rejects proposal execution.

One shared approval registry owns the bounded full/raw input caches and protected
vote data. This application validates all request fields, parent/config/profile
and logical retention charge before publication. Capacity, missing data, stale
parent and unavailable resources produce an unavailable node path, not a fabricated
execution-invalid vote or a fallback permit. Codec/crypto operations and a matching
root alone cannot construct an execution approval.

Replay metadata is a versioned domain-specific protobuf envelope over the existing
opaque RocksDB history, not an additional generic WAL. It binds canonical parent/
target version bytes, genesis/config/profile, native current/preceding hash,
proposer, exact FinalizeBlock input and complete commit identity. Native decided
input is synced before the canonical state commit. A second synced record references
that exact decision after the actual full-state acknowledgment.

Restart preserves a decision whose state is still at its parent and reports the
old height until the authenticated engine replays Finalize/Commit. If state already
matches a decided target but completion acknowledgment was lost, reconciliation
verifies the retained canonical identity and appends the completion marker. Missing,
noncontiguous or contradictory history refuses startup and fences the shared
signer. Exact already-applied Finalize/Commit replay returns retained results and
never charges fees twice or lowers the current head.

Cache/service guards are dropped by their owning storage operations before I/O.
This application is a single write actor; public query consumers use the separate
immutable StateService. Configured estimates bound logical retained state, raw
inputs, journals/results and execution reservations; they are not an allocator/
RSS guarantee or a secured throughput result. Ambiguous/fatal commit paths fence
the application and signer until namespace reopen/reconciliation.

Private unit tests run real canonical EVM/state/repository operations and use real
test-child Unix peer proofs for callback guards. Controlled request fixtures label
their local-engine assumption; they are not four-validator consensus results.
Unit fault hooks cannot enter production configuration. Tests distinguish simulated
lost continuation after actual sync from a child process exiting after real state
sync without destructors. Neither certifies hardware power-loss durability.
All raw output/data remains in ignored `local-tests/b3-preparation/`.

Initial production behavior returns no validator updates. Native H+2/H+3 transition
acceptance requires the separately reviewed authenticated development-only fixture
adapter; this domain exposes no arbitrary operator update schedule or B5 staking
interface. Complete four-process T-C01–T-C10 and B3 integration evidence remain
the root acceptance responsibility.

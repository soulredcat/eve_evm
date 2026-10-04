<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Verified execution status

Updated: 2026-10-04. This public summary records verified facts at the checkpoint
below. Documentation edits do not rerun or extend its runtime acceptance.

## Accepted B3 classical development gate

The complete local gate passes 545 cases at clean `28ffac3`, and
[hosted PR run 37096714168](https://github.com/soulredcat/eve_evm/actions/runs/37096714168)
passes 545 at `9d3e1d5`. Job 111128025705 succeeds in every required step.
The separate push run 37096712477 is cancelled; it supplies no passing evidence.
The verified B3 source is integrated with documentation-only acceptance updates.
B3 is integrated on main at c41ba2f351f17ad349efccb1e447e49822480c99.
B4 is actively in progress; its complete gate remains unpassed.

This accepts the classical development B3 contract, including real consensus,
execution, signer durability and historical transitions. It does not accept PQ,
stronger adversary continuity, secured capacity or independent-host deployment.

## Historical runtime verification baseline

Owner-authorized main checkpoint:
`c2f9b2ad22c69ea52c6b65a027e0bea5929412c8`.
[PR #3](https://github.com/soulredcat/eve_evm/pull/3) is observed merged/closed.
Publication was explicitly authorized despite the unresolved hosted failure;
it does not establish hosted B3 acceptance.

| Verification | Observed result at this revision |
|---|---|
| Complete local `cargo xtask verify --bulk B3` | PASS: 531 passed, zero failed/ignored/pending; 2448.798 seconds |
| Local consensus acceptance | 23 passed |
| Local validator tests | 101 passed |
| Local tooling tests | 148 passed |
| Retained local foundation/RPC/recovery tests | 259 passed |
| Local format, strict Clippy and workspace release build | PASS |
| Local structure / ownership | 1,543 / 1,577 files checked; zero violations |
| Hosted PR run 36954985597 | FAILURE; shared process scenarios failed, reason unclassified |
| Hosted branch run 36954982616 | FAILURE; native process image inode mismatch reported |

The successful local frozen source identity is
`44cde768033efd15fe0127ef0a8b55ddb08f7590c664edda8dd6aa8cfcf4e720`;
source identity was unchanged before/after the gate. The raw report is local-only:
`local-tests/verify-199-1790907529661361346/report.json`. A GitHub clone does not
contain that raw artifact. Earlier 518/525-case passes and the failed/aborted
intermediate attempts remain historical evidence, not replacement current results.

Hosted evidence is available in the
[failed PR run](https://github.com/soulredcat/eve_evm/actions/runs/36954985597),
[failed branch run](https://github.com/soulredcat/eve_evm/actions/runs/36954982616)
and [main run](https://github.com/soulredcat/eve_evm/actions/runs/36961090642).
The failed runs reached 259 preceding accepted cases before all ten real-node
scenarios failed. Do not infer that every run has an identical root cause.

## B3 repair and revision-bound evidence

Owner resumed work on 2026-10-03. B3 local and hosted acceptance are verified.
[PR #5](https://github.com/soulredcat/eve_evm/pull/5)
is based on the runtime baseline and includes bounded executable-transition
handling, direct test-process signaling, safe T-C07 phase diagnostics and strict
native committed-submission validation. Shared main documentation is retained.

| Candidate evidence | Actual result |
|---|---|
| `bc6c390`: complete local B3 gate | PASS: 533 tests, zero failed/ignored/pending |
| `bc6c390`: hosted run 36970778145 | FAILURE; registered T-C07 was unclassified |
| `db48da3`: complete local gate attempt | FAILURE after 385 accepted cases; new tooling test registry prefix was incorrect |
| `7d1ae09`: corrected tooling inventory | PASS: 149 tooling cases; complete corrected 534-case local gate NOT_RUN |
| `7d1ae09`: hosted push run 36988335828 | FAILURE after 259 preceding accepted cases; T-C07 reports `B3_TC07_SUBMIT` |
| `7d1ae09`: hosted PR run 36988343126 | CANCELLED; cancellation is not acceptance |
| Submission source committed as `09a22b9`: complete consensus packet | PASS: 28 passed, zero failed/ignored/filtered; 796.83 seconds |
| `09a22b9`: scoped validation before commit | Format and strict acceptance/tooling Clippy PASS; structure 1,557 / ownership 1,591 files, zero violations |
| `9362b22`: complete local B3 gate | PASS: 539 tests, zero failed/ignored/pending; 2581.231 seconds |
| `9362b22`: hosted push run 37086413900 | PASS: 539 cases; every required job step succeeds |
| `9362b22`: hosted PR run 37086416848 | FAILURE after 259 preceding accepted cases; T-C07 rotation reports RPC read timeout |
| `28ffac3`: complete local B3 repair gate | PASS: 545 tests, zero failed/ignored/pending; 2492.479 seconds |
| `9d3e1d5`: hosted PR run 37096714168 | PASS: 545 cases; every required job step succeeds |
| `9d3e1d5`: hosted push run 37096712477 | CANCELLED; not a pass |

The [classified failed PR](https://github.com/soulredcat/eve_evm/actions/runs/37086416848)
reports `B3_TC07_SUBMIT_ROTATION`, `B3_RPC_READ` and `B3_RPC_IO_TIMEOUT`.
This establishes a rotation RPC read timeout, not the underlying host slowness.
The new candidate supplies separate fixed rotation/leave/jail and inner RPC
categories without exposing raw payloads. It requires explicit successful native
admission/execution codes, the expected transaction SHA-256 hash and a canonical
positive height. Five new negative/deadline tests supplement the existing packet.
No deadline, quorum, durability or receipt assertion is waived. The unchanged
three-second absolute RPC deadline is tested.

The local repair uses one-shot CheckTx admission for T-C07, then observes exact
transaction bytes in native/application-covered blocks and validates the matching
native execution result by height/count/index/code. One existing 90-second budget
continues through H+3; per-RPC limits and all certificate/replay/receipt assertions
remain. Generic scenarios retain their original broadcast-commit path. Scoped
transaction tests pass 9/9, RPC 5/5 and tooling-result tests 2/2; strict scoped
Clippy/format and source review pass. Exact live T-C07 passes 1/1 in 88.95 seconds.
Complete local `28ffac3` and hosted PR `9d3e1d5` verification pass 545 cases.
Raw outputs stay local.

The clean `28ffac3` gate binds source
`0c33f0b1a16a14201edddc0f83613076b10b836cf1fa1521f24f7917a4e21c85`, unchanged
before/after testing. All 64 registered commands exit zero, including format,
strict lint and release build. Counts are 259 foundation, 34 consensus, 103
validator and 149 tooling cases, with zero ignored or pending. Local-only report:
`local-tests/verify-233-1790998425726437402/report.json`. This is classical
single-host development evidence; it does not accept PQ or secured capacity.

Local `bc6c390` source identity was
`f98eff89db5fd4db92529555fd4e1716142d3a40025d782bc33681c5d55bb9e6`,
unchanged across its 2579.064-second gate. Raw reports and intermediate failures
remain local-only. Earlier failed candidates do not replace accepted repair evidence.

The complete local `9362b22` gate tested a clean checkout with frozen source
identity `f078f05217f7effd440a4f5465bc3c530c9aca6bf629a0a0b52881dd2f177a56`,
unchanged before/after verification. It includes 259 retained foundation cases,
28 consensus cases, 103 validator cases and 149 tooling cases. Format, strict
workspace Clippy, structure/ownership and workspace release build pass. The raw
report remains local-only at
`local-tests/verify-227-1790991119710668573/report.json`; GitHub clones do not
contain it. Hosted verification remains required before main runtime integration.

### Historical executable-binding diagnosis

The classified branch failure includes `ENGINE_IMAGE_BINDING_FAILED`,
`ENGINE_PROCESS_INODE_MISMATCH`, `IO_PERMISSION_DENIED` and validator exit during
engine discovery. The inode flag means the process executable metadata was read
and differs from the trusted held file descriptor's inode. The guard creates
`PermissionDenied` for this rejection; it is not proof that the OS denied reading
the process image. The actual filesystem/container/launch cause remains unproven.

Start by inspecting these validator-owned paths:

- `validator/src/development/engine/verification/validate_engine_image_binding.rs`
- `validator/src/development/engine/verification/engine_process_image_identity.rs`
- `validator/src/development/engine/lifecycle/start_engine.rs`

Reproduce the pinned hosted environment or collect bounded reviewed diagnostics
that distinguish held descriptor identity from running process identity. Retain
digest, device, inode, length, modification/change-time authentication and all
signing/quorum/durability guards. Do not grant privileges or weaken checks based
on an unverified hypothesis. Unknown diagnostics remain redacted; raw logs stay local.

Then run scoped rejection/positive lifecycle tests and the complete B3 gate at
the repair SHA, and verify hosted CI. Report results separately by exact revision.
See [development instructions](../development/README.md) for tooling and review.
B4 foundations are developed separately; they do not close B3 or its prerequisites.

## B4 local development

Status on 2026-10-04: B4 IN_PROGRESS, local HEAD 88e7c82 with subsequent uncommitted
source, not pushed or integrated into main. Canonical public/master followers,
segmented recovery, private authenticated checkpoint import, resumable content/proof
storage, typed durable-base activation and default public checkpoint restart now
exist locally. Fixed genesis owners/profile are explicit; dynamic transitions
are not accepted by these follower/checkpoint APIs.
See [B4 scoped implementation and exact local evidence](b4/README.md) for actual
four-validator process cases, nonempty simulated tail loss, bounded N09 measurements
and remaining requirements. Public storage limits are two jobs, 16 MiB staging
and cooperative 2500-basis-point writer pacing, not OS hard quotas.
The B4 manifest/registry/workflow is executable source; complete local/hosted B4
verification and publication are NOT_RUN. T-N09/T-N10 completion remains pending
full-scope review. Retain all prerequisites and require hosted acceptance before main.
## Remaining acceptance

| Scope | Status |
|---|---|
| B0/SEC0, B1 and B2 | Historical complete local development gates passed; retain regressions |
| B3 | DONE for the classical development contract: local and hosted 545-case gates pass |
| B4 | IN_PROGRESS; implemented runtime/checkpoint slices; complete local/hosted gate NOT_RUN |
| B5–B11 | NOT_STARTED |
| SEC1 / SEC3 | NOT_STARTED; core security requirements remain mandatory |
| Independent copied public/validator distributions | NOT_IMPLEMENTED / NOT_RUN |
| DEVNET_ACCEPTED / SECURITY_PROFILE_ACCEPTED | NOT_ACHIEVED |
| PQ_PROFILE_VERIFIED / CONSENSUS_RESILIENCE_TESTED | NOT_ACHIEVED |
| 51_PERCENT_CONTINUITY | UNSATISFIED_BY_BASELINE |
| SCALE_TARGET_VERIFIED at 1M aggregate finalized TPS | NOT_ACHIEVED |
| SEC2 / INT0–INT3 and external programs | DEFERRED_UNTIL_EVE_TESTNET and later owner scope |

The classical development baseline is not a production, PQ, majority-immunity,
audit or capacity claim. No mainnet launch or real-value deployment is authorized.

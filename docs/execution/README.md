<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Verified execution status

Updated: 2026-10-02. This public summary records verified facts at the checkpoint
below. Documentation edits do not rerun or extend its runtime acceptance.

## Published checkpoint

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
| Hosted main run 36961090642 | IN_PROGRESS at the documentation observation; no final result claimed |

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

## Current B3 repair

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
B4 is not started while this prerequisite remains unresolved.

## Remaining acceptance

| Scope | Status |
|---|---|
| B0/SEC0, B1 and B2 | Historical complete local development gates passed; retain regressions |
| B3 | Local complete gate passed; hosted acceptance unresolved |
| B4–B11 | NOT_STARTED |
| SEC1 / SEC3 | NOT_STARTED; core security requirements remain mandatory |
| Independent copied public/validator distributions | NOT_IMPLEMENTED / NOT_RUN |
| DEVNET_ACCEPTED / SECURITY_PROFILE_ACCEPTED | NOT_ACHIEVED |
| PQ_PROFILE_VERIFIED / CONSENSUS_RESILIENCE_TESTED | NOT_ACHIEVED |
| 51_PERCENT_CONTINUITY | UNSATISFIED_BY_BASELINE |
| SCALE_TARGET_VERIFIED at 1M aggregate finalized TPS | NOT_ACHIEVED |
| SEC2 / INT0–INT3 and external programs | DEFERRED_UNTIL_EVE_TESTNET and later owner scope |

The classical development baseline is not a production, PQ, majority-immunity,
audit or capacity claim. No mainnet launch or real-value deployment is authorized.

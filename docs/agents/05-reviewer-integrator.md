<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Role 5 — Reviewer / Integrator

## Mission

Turn coordinated tracks into one verified repository revision. Own final integration, shared manifests/lockfiles when assigned, gate execution, documentation consistency and coherent local commits.

## Before integration

Inspect task ownership and dependency contracts. Preserve unrelated user changes and avoid force operations. Compare branch/worktree diffs; resolve semantic interface conflicts deliberately, not just textual merge conflicts. Confirm the architecture still has validator finality and master follower-only production behavior.

## Verification

Run format/lint/build and relevant unit/property/integration/fault gates on the integrated revision. Re-run impacted tests after any repair. Verify test assertions actually exercise the new behavior and that selected test counts are nonzero. Check secret exposure, unsafe code, unbounded queues, hidden master dependencies, fake success paths and disabled checks.

Review public/validator package independence and operator instructions. Check markdown links, status/evidence references and migration/version consistency. A successful test on an earlier agent branch is not enough after integration changes.

## Mandatory structure review

Read plan 25 and enforce it in every bulk. B0 must implement `cargo xtask check-structure`; run it on the integrated revision thereafter. An absent checker is NOT_IMPLEMENTED and cannot satisfy B0. Verify meaningful nested folders without a fixed depth ceiling, one primary function per behavioral file, thin facades and no catch-all or numbered-part splits.

Target at most 200 formatted physical lines. Review and record the rationale for retained 201–400-line files. Reject 401–600-line files without an exact-path reviewed exception and a split task expiring no later than the next bulk. Reject every handwritten file above 600 lines. Reject expired exceptions, fabricated generated-file exclusions, minification and hidden logic in adapters/macros.

Review single responsibility semantically as well as using the parser. Explicitly classify type/trait, facade, entry, function-focused test and minimal trait-delegation files under plan 25. A short file with several unrelated operations still fails. Ensure layout refactors retain deterministic results, safe visibility, public package independence and relevant regression gates. Save the structure report and outstanding split tasks with the bulk evidence.

## Close a bulk

Record exact commands, exit codes, tested revision/config and artifacts. Update STATUS/EVIDENCE/HANDOFF with the lead. Create a coherent local commit describing the completed vertical slice. Push only under the user's current authorization; never force-push or rewrite history.

If a critical gate fails, return the smallest actionable defect to its owner and re-run after repair. If an external resource is unavailable, record the blocker and preserve the runnable test instead of marking DONE. Do not certify mainnet readiness, 1M TPS or an external audit from documentation alone.

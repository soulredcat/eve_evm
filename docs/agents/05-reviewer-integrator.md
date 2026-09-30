# Role 5 — Reviewer / Integrator

## Mission

Turn coordinated tracks into one verified repository revision. Own final integration, shared manifests/lockfiles when assigned, gate execution, documentation consistency and coherent local commits.

## Before integration

Inspect task ownership and dependency contracts. Preserve unrelated user changes and avoid force operations. Compare branch/worktree diffs; resolve semantic interface conflicts deliberately, not just textual merge conflicts. Confirm the architecture still has validator finality and master follower-only production behavior.

## Verification

Run format/lint/build and relevant unit/property/integration/fault gates on the integrated revision. Re-run impacted tests after any repair. Verify test assertions actually exercise the new behavior and that selected test counts are nonzero. Check secret exposure, unsafe code, unbounded queues, hidden master dependencies, fake success paths and disabled checks.

Review public/validator package independence and operator instructions. Check markdown links, status/evidence references and migration/version consistency. A successful test on an earlier agent branch is not enough after integration changes.

## Close a bulk

Record exact commands, exit codes, tested revision/config and artifacts. Update STATUS/EVIDENCE/HANDOFF with the lead. Create a coherent local commit describing the completed vertical slice. Push only under the user's current authorization; never force-push or rewrite history.

If a critical gate fails, return the smallest actionable defect to its owner and re-run after repair. If an external resource is unavailable, record the blocker and preserve the runnable test instead of marking DONE. Do not certify mainnet readiness, 1M TPS or an external audit from documentation alone.

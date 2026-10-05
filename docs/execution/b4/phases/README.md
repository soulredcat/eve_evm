<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# B4 phased checkpoint

Checkpoint on 2026-10-04: PAUSED_BY_OWNER. B3 remains the accepted runtime on main
at c41ba2f351f17ad349efccb1e447e49822480c99. The owner authorized publication
of completed records and draft PRs for unfinished B4 work. No B4 phase currently
has complete revision-bound local and hosted acceptance; none is marked DONE.
This document records progress, not permission to resume development.

The 2026-10-05 collaboration checkpoint has a complete local P1 PASS: 743 cases
at clean ca32145. The owner requested pause after that gate. P1 still needs
hosted acceptance and main integration. See [verified P1 evidence](../phase1/README.md).

## Review phases and integration order

The phases follow existing coherent local commits. They are review/publication
boundaries within B4, not new bulks, waived requirements or security profiles.
The PRs are stacked so each dependent diff can be reviewed separately.

| Phase | Draft PR / base | Frozen implementation boundary | Implemented source | Acceptance status |
|---|---|---|---|---|
| B4/P1 | [PR #6](https://github.com/soulredcat/eve_evm/pull/6), base main | 5713c18 | Locally anchored native/application history, compact replay, authenticated journal import and charged public RAM modes | Complete local743 PASS at ca32145; hosted acceptance/main integration pending |
| B4/P2 | [PR #7](https://github.com/soulredcat/eve_evm/pull/7), base P1 | 88e7c82 | Bounded segmented recovery/WAL markers, actual public following and protected master archive | Pending P1 and exact-revision runtime/gate verification |
| B4/P3 | [PR #8](https://github.com/soulredcat/eve_evm/pull/8), base P2 | f10482e plus pause documentation | Resumable checkpoint bootstrap/reopen, real resource admission/pacing, retention/index pressure and broader recovery faults | Component checks pass; complete B4 gate failed before tests; hosted acceptance pending |

Merge only an accepted phase, in P1 -> P2 -> P3 order. Retarget a dependent PR
against the integrated base as needed without rewriting published history.
The inherited B3 workflow on P1/P2 is a retained regression packet; it does not
by itself select every new finality/sync component test and cannot certify the
whole phase. Complete applicable inventories and bind evidence before merge.

These are the ordered execution phases that exhaust B4, not optional review
labels. Complete P1 before P2 and P2 before P3. A phase is DONE only after its
implementation, required tests/integration/docs, exact-revision local/hosted gate
and main integration are complete. P3 includes the full integrated B4 closure
gate; that gate is part of P3, not a fourth phase. Once P1-P3 are DONE and the
full B4 packet passes, mark B4 DONE and proceed to the next existing bulk only
when the owner resumes. Do not invent additional B4 phases or omit requirements.

| Required B4 coverage | Owning phase |
|---|---|
| Canonical trust/identity/history, replay/import and commitment mutation guards | P1, retained and rerun through P3 |
| Actual public bootstrap and master-offline catch-up, segmented WAL and coherent RAM/durable markers | P2, retained and rerun through P3 |
| Resumable checkpoints, interruption, bounded hostile inputs and KEEP_ALL retention | P3 |
| Slow/failing storage interference, unsynced-tail recovery, complete replica-loss and commit-boundary faults | P3 |
| Complete T-N01-T-N07, T-N09/T-N10, T-S04-T-S08 and T-G04/T-G06 integrated closure | P3 |

## What is already verified

The source packets combined into f10482e have these integrator-run local results.
They do not silently certify earlier P1/P2 revisions or the complete B4 contract.

| Check | Recorded result |
|---|---|
| Public release suite after maintenance integration | 150 passed, zero failed/ignored; 9.71 seconds |
| Tooling suite | 164 passed; strict lint passed |
| Public strict all-target Clippy | Passed |
| Structure / Redcat ownership | 2,907 / 2,941 files checked; zero violations before pause documentation |
| Real four-validator CLI slices | Public bootstrap/restart, checkpoint interruption, master catch-up, SIGKILL tail and two configured replica-loss recoveries passed individually |
| Extended local N09 fixture | 128 checkpoint jobs, 128 KEEP_ALL compactions and 128 secondary-index jobs passed with paused WAL and production HTTP RPC |
| Pressure balance / eth_call p99 | 0.443699 / 1.080138 ms; sampled RSS 22,716,416 bytes |

These are bounded single-host CLASSICAL_DEV measurements. They do not prove
sustained TPS, OS resource quotas, unsampled memory peaks, physical power-loss,
geographic availability, independent master replication or quantum security.
Earlier scoped packets are retained in phase source history and PR descriptions.

## What remains unfinished

P1 now has a complete local packet selecting its 85 verifier cases and all
inherited core/security/role/structure/ownership cases. Hosted acceptance for the
complete candidate and main integration remain pending. P1 has no download
client package; that source appears in P2. A green 658-case repair job alone
cannot certify the complete 743-case P1 boundary.

P2 needs accepted P1, exact-phase public/master CLI catch-up and durable-prefix
restart/guard fixtures, complete phase-local inventories, local verification and
hosted acceptance. Later combined-candidate CLI results remain separately scoped.

P3 needs accepted prerequisites and a complete B4 packet. The first local attempt
at clean f10482e failed before executing tests because the managed worktree
provisioned-tools.json receipt was absent. Source, structure and ownership checks
passed in that attempt, but zero tests executed, so the packet is FAIL.

Canonical tool provisioning completed Go/Comet extraction, Comet build/module
verification, OpenSSL configuration/build and Node extraction. The owner pause
stopped client installation before the complete receipt was written. The empty
owned provisioning lock was removed only after all task-owned actors ended.
Tool receipts, source archives and partial client data remain ignored locally.

When the owner asks to continue: complete canonical tool provisioning, verify the
phase inventories, run their complete local/hosted gates and integrate accepted
phases in order. Before closing B4, run the full cargo xtask verify --bulk B4
contract, retaining T-N01-T-N07, T-N09/T-N10, T-S04-T-S08 and T-G04/T-G06.
Never omit a test, weaken a resource cap or grant master finality to obtain a pass.

## Pause and publication boundary

All local task-owned build, test, role and provisioning actors have stopped.
Unrelated programs were preserved. No local background watcher or automatic
resumption is scheduled. GitHub workflows triggered by the authorized checkpoint
publication may run externally; their eventual result is not claimed here.

Main receives the reviewed progress record. Unaccepted B4 runtime source remains
in the draft phases. Raw logs, credentials, build/dependency output, machine
configuration and temporary data are never published. First-party shared prose
remains English; tracked Markdown remains limited to files named README.md.

B5-B11 and required SEC1/SEC3 follow their existing dependency gates after the
owner resumes. B6 standalone role-copy and B8 independent-master acceptance remain
unfinished. D40 defers external-chain programs until EVE testnet; no new bridge
runtime or production custody is introduced by this checkpoint.

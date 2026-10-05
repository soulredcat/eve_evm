<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# B4 phase 1 collaboration and gate verification

Status on 2026-10-05: ACTIVE, acceptance pending. Codex's integration branch is
codex/b4-work. It combines Claude's published P1 source a539d04 (including the
four storage registrations in 7ee93f8) with the reviewed main documentation.
Claude owns RPC submission changes and the original-versus-fixed node comparison.
Codex owns complete gate selection, verification, review and final integration.
No passing phase or complete B4 result is claimed from source review alone.

## Selected phase contract

P1 contains finality verification, compact replay, journal import and charged
public RAM modes. It has no sync-client or development-fixtures package.
Its existing finality-verifier-b4.toml group now joins all retained B3 groups.
The manifest declares 743 unit/integration cases before registered doc tests.
The existing verify command is retained with an explicit phase profile:

    cargo xtask verify --bulk B3

This is the complete P1 packet retaining B3, not full B4 acceptance. B4 remains
NOT_IMPLEMENTED on this phase revision. P2 additionally selects its actual sync
client and verifier inventories. P3 includes the complete B4 closure gate.
Do not copy later-phase inventories into P1 or omit existing core/security,
structure, ownership, role, tool/compiler, format, lint or release-build checks.

## Executed checks and current failure

The integrator ran pinned Linux test listings with two build jobs. Exact selected
inventories match: storage60, state61, public48, verifier85, validator103 with
development-acceptance, and consensus acceptance34. These are inventory results,
not execution passes. Format checking passed before the compile-fixture repair.

Nine transaction decoder/admission/observation regressions passed in4.37 seconds.
They retain code/hash/height/index checks, one-shot admission and late-result
refusal. Per-request3-second/connect1-second limits remain unchanged; the helper
now gives the other scenarios an explicit90-second submission-progress budget.
It uses the existing maintained T-C07 path. Independent certificate/replay and
scenario-specific consensus/signing assertions remain mandatory.

The first full verifier execution failed one compile-time privacy case. It
expected E0502 (mutating bytes while their borrowed preflight remains live), but
independently selected state rlibs had different feature identities and produced
E0308 instead. The expected error and unrelated-error rejection remain mandatory.
The fixture now infers its budget from the actual verifier API and avoids an
unneeded independent state dependency. The exact E0502 regression passes and all
85 verifier cases pass; strict verifier/submission lint passes. Expected-error
and unrelated-error assertions are unchanged. Complete P1 acceptance is pending.

## Timing evidence and remaining work

A commit-waiting RPC can exceed a short request deadline under scheduling or
consensus delay. Equal3-second propose and request defaults alone do not prove
the observed failure: startup waits height2 before scenario submission, and a
complete rejected proposal need not consume the propose timeout. Claude's
comparative node timings/traces must establish any more specific cause.

The compile boundary repair and all 85 verifier cases pass; nine submission
regressions and 149 tooling cases pass, with strict scoped lint. Mandatory
structure and ownership checks have zero violations. These are component checks.
Verify all selected inventories/fixtures,
freeze a coherent revision, pass the complete local phase packet and hosted
acceptance, then integrate P1. Apply the accepted changes to dependent phases
normally without rewriting history. No P2/P3 source or accepted main runtime
is changed by this local candidate. Root's raw output remains ignored under
local-tests/b4-phase1-coordination; only reviewed English summaries are published.

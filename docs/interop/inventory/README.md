<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Historical INT0 interoperability inventory

This inventory preserves B0/INT0 facts at historical source revision
`038fe80f412754e5a7080240ca0e57ba3c19e368`. INT0 froze compile-tested metadata
and identified source-verifier requirements; it did not implement authenticated
source history, custody effects, SDK signing, endpoints or deployment approval.

Under D40, INT0–INT3 and SEC2 are `DEFERRED_UNTIL_EVE_TESTNET`. They are outside
current EVE core gates; reaching testnet requires subsequent owner scope before
a separate program may begin. Deferral is neither a current core failure nor a
completed route. No replacement adapter prototype or scaffolding is authorized.

- Version-one encoding and fixture identities (local-only: `encoding-and-fixtures.md`)
- Exact source and tool pins (local-only: `source-and-tool-pins.md`)
- Per-direction proof feasibility and blockers (local-only: `source-proof-feasibility.md`)
- Acceptance outcomes and coverage (local-only: `gate-outcomes.md`)

Historical source paths were `public/components/interop/` for metadata and
`validator/components/bridge-protocol/` for disabled bridge-profile contracts.
They are removed from active core rather than retained as unused dependencies.
Consult the recorded revision to inspect or reproduce that source. Future
programs adapt to EVE's versioned public evidence/transaction APIs and own their
remote clients, routes, verifiers and custody; core need not know external chains.
Master has no bridge approval, custody or source-finality authority.

The historical metadata represented Ethereum -> EVE, EVE -> Ethereum,
Solana -> EVE and EVE -> Solana, all VERIFIER_INCOMPLETE and DISABLED_NOT_APPROVED.
Those directions are now deferred candidates. No metadata test produced a
VerifiedBridgeMessage or authenticated finality.

The type/codec/amount/registry tests cover foundational parts of T-I01/T-I02/T-I06.
Endpoint/client/proof/SDK/incident/package acceptance was NOT_IMPLEMENTED/NOT_RUN;
current program scope is deferred. External cryptographic assumptions were
classical and do not become PQ-secure by association. EVE's active paired
consensus profile still requires core SEC1 enforcement independently of this work.

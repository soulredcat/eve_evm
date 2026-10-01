<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# SEC0 security inventory

This inventory records authorization and commitment boundaries at B0. It is not
an audit, an activated network profile or SECURITY_PROFILE_ACCEPTED. Runtime
coverage remains explicit in the linked tables; a working primitive does not
close missing core consensus, account, recovery or EVE-client authorization.
Decision D40 retains SEC0/SEC1/SEC3 as core and marks SEC2/INT0–INT3
`DEFERRED_UNTIL_EVE_TESTNET` for subsequently authorized separate programs.

- [Authorization paths](authorization-paths.md)
- [Commitments and identity strength](commitments-and-identities.md)
- [Crypto sources and maintained reference](crypto-sources.md)
- [Adversary assumptions and expected outcomes](adversary-and-gate-outcomes.md)
- [Historical external-chain verification research](../../interop/inventory/source-proof-feasibility.md)

The canonical ML-DSA/paired authorization source belongs to
`validator/components/authentication/`; it does not implement validator finality,
trusted enrollment or production signing. Historical public route metadata at
`public/components/interop/` and disabled `validator/components/bridge-protocol/`
contracts are recorded at revision `038fe80f412754e5a7080240ca0e57ba3c19e368`
and removed from active core under D40. No future program scaffold is authorized.
EVE's own headers, receipts, finality and proofs remain core interfaces; later
adapters consume them without adding remote clients or custody to core.

Keep CLASSICAL_DEV, HYBRID_EXPERIMENTAL laboratory coverage and a future verified
profile separate. EVE/external-chain authentication assumptions do not change the
CometBFT less-than-one-third Byzantine baseline. Required 51% continuity remains
UNSATISFIED_BY_BASELINE.

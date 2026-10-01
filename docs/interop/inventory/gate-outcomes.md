<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Historical interoperability gate outcomes and B0 coverage

This table preserves original expected outcomes and actual B0 subsets at source
revision `038fe80f412754e5a7080240ca0e57ba3c19e368`. Under D40, T-I01–T-I12 and
INT0–INT3 are `DEFERRED_UNTIL_EVE_TESTNET` for a subsequently scoped separate
program, outside current core gates. The old NOT_IMPLEMENTED/NOT_RUN values below
describe historical route coverage; they are not failed current core acceptance.
No test ID is closed by a foundation subset, deferral or a registered future case.

| Gate | Required complete outcome | B0 foundation / remaining work |
|---|---|---|
| T-I01 | Reject wrong genesis/address namespaces, truncation and wrong recipient/token accounts | Raw/canonical codecs, chain/asset/custody/recipient metadata checks and byte vectors pass; real custody/token-account derivation still absent |
| T-I02 | Reject unknown/incomplete/paused routes; isolated mock adapter does not advertise an integrated chain | Bounded discovery/duplicates/unknown routes and all disabled states pass; actual registry/runtime and adapter isolation later |
| T-I03 | Real Ethereum fake native/ERC-20 lock/mint/burn/unlock with source verification and destination contracts both ways | NOT_IMPLEMENTED |
| T-I04 | Real Solana fake native/SPL two-way custody with program/account/PDA/mint/token/authority checks | NOT_IMPLEMENTED |
| T-I05 | Nonfinal/stale/fork/wrong-root/failed-event proof cannot move value; RPC labels insufficient | No verifier/value movement exists; NOT_IMPLEMENTED |
| T-I06 | Exact decimals/zero/dust/overflow/native wrapping/multi-route conservation, reject unsupported tokens | U256/u64 scale and metadata rejection tests pass; custody wrapping/conservation later |
| T-I07 | Retries/relayers/revert/crash/delay preserve exactly-once effects without timeout refunds | NOT_IMPLEMENTED |
| T-I08 | Typed SDK/wallet/signing/recipient/fees/gas/account creation/resume/manual claim work | NOT_IMPLEMENTED |
| T-I09 | Actual destination enforces EVE historical active profile; external classical dependencies remain visible | Inventory present; actual destination/profile verification NOT_IMPLEMENTED |
| T-I10 | Real proof/gas/compute/size/staging/invalid-input/backlog limits measured | Foundation codec/registry/amount bounds only; actual proof costs NOT_RUN |
| T-I11 | Upgrade/provider/extension/pause changes cannot bypass approval or backing | Disabled metadata and Token-2022 rejection pass; real incident/admin runtime absent |
| T-I12 | Reproducible standalone packages run both integrations and report application/authentication/approval separately | NOT_IMPLEMENTED / NOT_RUN |

Historical test binaries in `public/components/interop/tests/` at that revision:
`chain_identity` (6), `asset_origin` (4), `exact_amount` (4),
`route_capabilities` (7), `transfer_request` (3), `local_bridge_identity` (2):
26 tests, no ignored cases, latest run exit 0. The fixture uses synthetic genesis/raw addresses and
independently specified bytes, not Ethereum/Solana source-finality vectors.

Historical reproduction: inspect that revision in an isolated checkout,
then run `cargo test --locked -p eve-interop`; the package is absent from active
core. A future program must discover nonzero tests and reject missing requested
implementation rather than let this metadata subset impersonate route acceptance.
Current requests for deferred INT gates report the deferral, never a passing
empty selection. EVE execution/RPC correctness tests remain current core work.

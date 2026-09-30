# Interoperability gate outcomes and B0 coverage

Expected outcomes retain plan 30's mandatory two directions for both Ethereum and
Solana. No test ID is closed by a foundation subset or a registered future case.

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

Versioned test binaries in `public/components/interop/tests/`:
`chain_identity` (6), `asset_origin` (4), `exact_amount` (4),
`route_capabilities` (7), `transfer_request` (3), `local_bridge_identity` (2):
26 tests, no ignored cases, latest run exit 0. The fixture uses synthetic genesis/raw addresses and
independently specified bytes, not Ethereum/Solana source-finality vectors.

Reproduce: `cargo test --locked -p eve-interop`. The integration runner must
discover nonzero actual tests; zero library/doc-test cases alone cannot pass a
requested foundation gate. Full INT1–INT3 gates must reject missing implementations
rather than allowing the B0 metadata suite to impersonate them.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Authorization-path inventory

Status: B0 inventory; mandatory core integration is SEC1/SEC3 work. Existing
`eve-crypto` paired verification is a local primitive, without authenticated
enrollment or consensus-engine use. A caller-supplied paired identity is not a
trusted validator/account registration.

D40 defers bridge custody and external-chain client paths below to separate
programs, `DEFERRED_UNTIL_EVE_TESTNET`, with subsequent owner scope required.
They do not fail current core profile acceptance. Historical inventory remains
factual; core EVE follower/light-client, signer and recovery paths remain required.

| Boundary / operation | Initial assumption | Required protected authorization / missing integration | Gate |
|---|---|---|---|
| Validator proposal, prevote, precommit | CometBFT Ed25519 development path | Engine-native classical AND ML-DSA for the same enrolled identity/sign bytes, height/round/step/network; no post-finality stamp | T-P02/T-P03 |
| Commit/certificate verification | Historical weighted validator set | Count identity once after both components, strict quorum, executed proposal/data availability | T-M01/T-P02 |
| Validator-set enrollment, activation, retirement | Bonded power and authenticated historical transition | Possession of both keys; bound epoch/profile; previous-history authentication | T-P04/T-M07 |
| Durable signing and signer fencing | Not implemented as a runtime | Sync signing intent before vote; restart/retry identity; one active signer; reject corrupt/stale backups | T-M06/T-P06 |
| Legacy EVM EOA transaction and ecrecover | secp256k1 compatibility | Remains CLASSICAL; do not reinterpret Ethereum transaction/opcode semantics | T-P03/T-P10 |
| Protected account deploy/init/execute | Not implemented | Explicit ABI/gas/nonce/expiry; mandatory paired owner authorization over every operation field | T-P03/T-P05 |
| Approve/permit/session/delegate | Legacy contract behavior is classical | Bind allowance, spender, expiry, revocation and contract authority; no unprotected delegated owner | T-P05 |
| Protected account admin/upgrade | Not implemented | Same active profile on implementation selection, storage migration and owner changes | T-P05/T-P09 |
| Guardian/account recovery | Not implemented | Enrolled guarded authority and recovery delay; no classical-only bypass; migrate allowances and pending operations | T-P05/T-P06/T-P09 |
| Staking/delegation/withdrawal | Not implemented | Protected owner authority, nonce and delayed set activation; durable funds/ownership rules | T-P04/T-P05 |
| Slashing/evidence and unbonding | Historical identities / engine evidence window | Both components where activated, retention/trust consistency, no arbitrary accusation or guessed operator identity | T-M06/T-M07/T-P04 |
| Public/bootstrap/master-following clients | No authenticated runtime yet | Historical profile/set/anchor binding; verified replay or authenticated commitments; master has no finality key | T-M05/T-P07 |
| Stored checkpoints/snapshots | Storage checksums are not authentication | Trusted history/set/profile, replay tail and expiration; old classical anchors are not retroactively PQ | T-M08/T-P04/T-P07 |
| Master administration / endpoint access | Protected transport still unimplemented | Separate credentials, least authority, authentic transport; cannot vote, approve a route or rewrite history | T-M05/T-P09 |
| Core conflicting-finality handling | Observable authenticated history conflict | Quarantine affected authentication/readiness, preserve evidence; master cannot choose replacement history or clear conflict | T-M09 |
| Releases / upgrade control | Two-of-three development identities, not implemented | Both signatures per counted identity, manifest/artifact binding, anti-downgrade; separate from consensus/custody | T-P09 |
| Transport authentication | TLS/P2P credentials not selected/activated | Maintained protocol, authenticated peers and downgrade protection; classify remaining classical certificates | T-P09 |
| Transport key exchange | Not selected/activated | ML-KEM is key agreement, not signatures; use maintained hybrid transport and record confidentiality separately | T-P09 |

## Deferred separate-program paths

These rows are future program obligations, not current core dependencies or
claims of completed custody. Any authorized adapter must conform to EVE's public
versioned evidence and ordinary authenticated transactions. Remote-chain
availability/speed cannot control EVE block production, voting, finality or sync.

| Boundary / operation | Historical initial assumption | Required future protected authorization | Deferred gate |
|---|---|---|---|
| Bridge deposit/burn user authority | Endpoints' actual account models | Protected EVE operation; disclose external classical wallet/source authority | T-BR09/T-I09; program extension of T-P05 |
| Bridge source-client updates / claims | No source verifier implemented | Actual source finality/inclusion/historical profiles; atomic replay/backing checks | T-BR01–T-BR05/T-I05 |
| Bridge custody/token mappings | No custody runtime implemented | Chain-specific contract/program authority, exact origin/deployment and conserved backing | T-BR07/T-BR08/T-I06 |
| Bridge pause/resume/recovery | No incident runtime implemented | Pause-only authority; fresh-anchor reconciliation for resume; no emergency mint/unlock | T-BR06/T-BR10; program extension of T-M09 |
| External consensus/admin | External classical assumptions | Route-specific verification/upgrade controls; EVE cannot upgrade external key security | T-BR09/T-I09 |

No row is accepted merely because it appears here. Existing standard signatures
and external BLS/Ed25519 remain vulnerable to a sufficiently capable quantum
adversary. Even complete paired authentication cannot stop an authorized malicious
quorum. Production recovery/genesis/custody authority and source-license grants
remain owner decisions.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Canonical development fixtures

This validator-owned component contains the single shared implementation of the
first-party deterministic genesis, native signing and three-height recovery-chain
fixtures. Public, validator and master tests may consume it only as a dev-dependency.
Production components must not depend on this component or use its identities.

The genesis uses the existing four deterministic Ed25519 seeds, ClassicalDev profile,
chain identity and economics. The signed transaction bytes, funded sender, storage
contract, canonical native validator ordering, vote sign bytes and serial execution
context are preserved. These fixtures sign using the maintained consensus codec and
execute using the maintained EVE executor; no cryptography or execution rules are
copied. Seeded keys are unsafe test identities, not secrets or production credentials.

Public modules are genesis (genesis/funded_genesis), native (Frame/frame/resign),
transactions (sender/signed_transaction) and recovery (RecoveryChain/recovery_chain
and the preserved CLONE_BYTES executor parameter). Recovery also reexports the
funding and transaction operations for existing test adapters. Genesis is an alias
of the canonical DevelopmentGenesis type. No finality-verifier dependency is used.

Consumers perform only their own NativeFrame, envelope or wire projections. They
must not import source through paths escaping their package, duplicate the signer,
or treat fixture structs as authenticated finality capabilities. Component tests
check actual native certificates, mutation refusal, canonical genesis and full
serial state/receipt parity. Classical development evidence grants no PQ, mainnet,
production-finality, capacity or complete B4 acceptance claim.

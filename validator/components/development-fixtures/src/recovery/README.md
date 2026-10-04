<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Canonical recovery development corpus

Canonical owner: validator development fixtures, for dev-dependency consumers
only. One `build_recovery_chain` operation calls the maintained canonical state
executor and existing native signing/context fixtures. It does not implement
another executor or cryptographic primitive.

`recovery_chain()` preserves the existing nonce-zero transaction at H1 followed
by empty H2/H3, with the same raw transaction, genesis, context, native signatures
and canonical encodings. `recovery_chain_with_nonempty_tail()` uses the same
operation and inputs at H1, a nonce-one call by the same sender at H2, and empty
H3 as certified lookahead. Transaction selection is the only changed workload.
The maintained transaction signer has an exact nonce-zero equality test against
the preserved raw golden vector.

The [transaction capability](../transactions/signed_transaction_with_nonce.rs)
uses maintained Alloy transaction codecs and k256 signing with a publicly known
unsafe CLASSICAL_DEV corpus identity. That seeded identity is never an operational
credential, production default, PQ key or custody authorization. Native certificate
signers remain the existing canonical shared implementation.

New tests check the nonce-zero golden bytes, both recovered sender identities,
nonempty receipts, nonce-two state, full canonical commitments, unchanged first
height, storage effects and exact 40/30/30 fee conservation. Existing fixture
tests are preserved. The integrator must execute the tests before reporting a pass.

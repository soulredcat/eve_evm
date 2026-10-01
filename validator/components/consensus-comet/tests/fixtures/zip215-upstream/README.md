<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Preserved upstream ZIP215 edge inputs

This first-party provenance document describes separately attributed upstream
test inputs. Redcat permission-only terms do not relicense or restrict those
upstream inputs, mathematical results or their preserved LICENSE/NOTICE.

The twelve cases are public key/message/signature tuples from the pinned Oasis
curve25519-voi corpus `testdata/speccheck_cases.json.gz`, SHA256
`642794e72d13b11e19730019741c400f646696c03e55949f4bc4ae7204ee79e2`.
The exact native dependency is
`github.com/oasisprotocol/curve25519-voi@v0.0.0-20230904125328-1f23a7beb09a`.
Its preserved notice credits the paper "Taming the many EdDSAs" by Chalkias,
Garillot and Nikolaenko, and source revision
`336651ba7f1c1ae90b7deac7d175290863a00b66` of
`novifinancial/ed25519-speccheck/scripts/cases.json`.
The original Novi license is Apache-2.0 and remains exact in `LICENSE-NOVI`,
SHA256 `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`.
Oasis's derived-format implementation/test notices retain BSD-3-Clause conditions.
The composite projected JSON and preserved source NOTICE retain both relevant
licenses; neither overrides the other. The original license texts remain separate.

JSON projects public fields into the reviewed fixture schema and adds measured
acceptance/canonicality/order classifications from existing native APIs. No
private material, upstream filling script or custom production crypto is included.
The Rust test compares actual individual Zebra verification with actual pinned
Comet verification outcomes; it does not infer acceptance from a curve equation
implemented by the test itself.

Case nine has a canonical, non-small-order but mixed-order public key and a
noncanonical R. Native ZIP215 accepts it while strict Dalek rejects it; a weak-key
test alone would not expose this semantic difference. Other cases clarify that
raw native signature acceptance is distinct from EVE's safe enrollment policy.
Keep all upstream notices and exact input bytes when redistributing these fixtures.

This corpus verifies the native classical primitive boundary. It is not a PQ
test, enrollment bypass, economic security proof, audit or distributed devnet run.

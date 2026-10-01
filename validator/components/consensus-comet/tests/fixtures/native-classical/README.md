<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Independent native classical fixtures

Canonical owner: validator consensus component's independent protocol tests.
These fixtures contain original bounded schema/selection and public inputs/results
from exact CometBFT production APIs at revision
`0880b4d378f347ab16e54ec677ff50d803f37d62`. Upstream protocol and reference facts
retain their rights; the Redcat notice does not claim upstream implementation
ownership or withdraw upstream permissions.

The oracle imports native `types.VoteSignBytes`, `ProposalSignBytes`, `Header.Hash`,
`ValidatorSet.Hash`, key addresses and `ValidatorSet.VerifyCommit`. It runs with
the pinned Go toolchain, readonly module graph, trimpath and VCS stamping disabled.
Private signing material stays in memory; only public keys and signatures are
serialized. Fixed labels create deliberately unsafe fixture identities; they
cannot serve as runtime enrollment or real signing keys.

The locally preserved v3 aggregate has SHA256
`5fdadb9387dee3ae50f29c51f8bb6448b4a780ac1fc62c19d5679836c33e9999`.
The local source-identity record has SHA256
`99a68791e05d795251fb6be93f709a1ed9fb0bfdc7269d8f06132f640a2405b6`;
it records every Go oracle source and native go.mod/go.sum identity. The exact
native go.mod digest is
`bc57a84a4e1ad8c2a4da0d8e9e4577848fc9391b56d4a134845dd677b099417b`;
go.sum is `f9ade950dbd4332a5c9842c9ae601745c4dfe9139da3bb11d2d2c58fd4c5f8b6`.
Raw oracle source/output and receipts are explicitly local-only under ignored
`local-tests/b3-preparation/`; the complete public assertion inputs are versioned.

Per-case JSON preserves canonical bytes, numeric inputs and native outcomes.
Verbose native rejection text is replaced by the fixed
`INSUFFICIENT_VOTING_POWER` category without changing signed fields or outcomes.
No JSON comments or generated lock/header changes are used for ownership notices.

| Family | Assertion scope |
|---|---|
| votes | Nil/full blocks, year-one/Unix/full nanosecond time, maximum integer fields, chain domain and excluded routing/signature fields |
| proposals | No prior round, a valid prior round and Go-zero timestamp |
| sets | Native order, key address, equal/unequal power and non-power-of-two field trees; native weak/invalid-point byte hashing is separate from enrollment |
| data | Empty/one/three/five and reordered native transaction-ID trees; raw byte/count bounds remain explicitly development scoped |
| headers | Original upstream block-version-one hash reference is rejected by the version-11 wrapper; supported version-11 fields match native hash |
| certificates | Three-of-four with absent/nil fourth vote succeeds; exact two-of-three fails |

The extension-exclusion vote intentionally carries fields that the native
canonical serializer ignores. Production baseline encoding first rejects those
unsupported extension fields; after their removal, unchanged expected native
bytes verify the exclusion without treating an ignored extension as authorization.

Native signature verification and EVE enrollment are separate responsibilities.
The upstream ZIP215 vectors live beside this directory with their own attribution.
A certificate fixture authenticates its literal synthetic header; it supplies no
execution-validity proof, actual four-validator process run, trust-history proof,
signer-durability acceptance or B3 bulk completion.

Additional native weak/invalid-point set vectors come from the local standalone
`weak_identity_extra.go`, source SHA256
`c423c1d992d216ddf9616776e75311d122d79677dd8c0b6c44a8f3604c210f4d`.
They prove native hashing performs no curve-point admission. Invalid compressed
points still fail actual individual signature verification.
The data-hash oracle imports native `types.Data.Hash`; source SHA256 is
`05ea01f871db546d2c987d87e45869fc66de29626e6ae9e519c38ac3db377f7d`,
output SHA256 `bd1810d849bbde4add61773993ed8af63a21924f1616921f370a10e9cd727476`.
It uses the same readonly native module graph/toolchain with network fetching off.

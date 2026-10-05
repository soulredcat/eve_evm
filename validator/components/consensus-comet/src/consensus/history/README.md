<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Sequential native classical history

Validator-owned pure verification reused by public and master followers. It uses
the pinned native certificate, canonical set and transaction-data hash primitives.
There are no database, transport, signer or private master dependencies.

Initialize from validated local genesis or an explicitly trusted local native
header/block identity. A downloaded header is not a trusted checkpoint merely
because its hash matches. The caller must establish immutable EVE genesis/network,
profile/key-epoch provenance and checkpoint freshness before this local boundary.
Genesis initialization binds the first header to the configured height-zero
application hash and initial validator set. It constructs no ApplicationCommitment(0).

Each successor must have the exact next height and complete preceding block ID,
strictly increasing native timestamp, and a canonical applicable set whose hash
matches the preceding certified NextValidatorsHash. Updates produced at H appear
in H+1.NextValidatorsHash and become the active set at H+2. The existing certificate
checker verifies all signatures under that set and requires strict 3*S > 2*T.
Raw transaction bytes must match the certified native Data.Hash. Existing local
development validator-count and transaction byte/count limits remain enforced.
Duplicates, gaps and reordered deliveries reject; callers handle idempotent
delivery outside this verifier using previously accepted content identities.
Failures leave the accepted history unchanged.

Private fields prevent constructing verified-header results from arbitrary peer
hashes. Explicit trust initialization remains a caller authority boundary, not a
cryptographic authentication of that initial input. Results certify native header
history and returned transaction data only. They do not independently execute EVM,
authenticate arbitrary snapshots/deltas, prove freshest available history, validate
EVE proposer-owner mappings or establish profile/key-epoch transitions. Higher
EVE verification must authenticate post-state H through certified H+1 or replay,
check exact delta bases and derive application metadata from authenticated state.

Authentication is fixed to the caller-trusted supported profile at initialization.
Activated hybrid requirements fail closed through the existing native guard.
Classical development results are not PQ-secure; baseline safety assumes less than
one-third Byzantine voting power. Checkpoint trust-period rules and observable
conflicting-history quarantine belong to the higher EVE verifier/runtime.

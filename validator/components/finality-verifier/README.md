<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# EVE development finality and application anchoring

Canonical validator-owned pure verification consumed by role runtimes. This
component depends on native consensus verification, protocol configuration and
canonical logical state. It contains no master authority, database, signer, live
transport or execution engine.

Initialization consumes a locally selected DevelopmentGenesis and an actual state
budget. The existing initialize_development_state validates canonical genesis,
funding, enrollment and system records and computes the exact height-zero content
digest used by B3's native initial AppHash under D41. Network name is the actual
native chain ID. The immutable genesis digest binds EVM chain, configuration and
initial state. No caller-supplied root or validation flag creates this trust.

The wrapper retains its own private native history. Before native history advances,
it checks exactly 32 application-hash bytes and the native application version
actually declared by B3's genesis consensus parameters and Info response. Known
scheduled upgrade activation fails closed because this first slice implements no
upgrade execution or profile transition. Native sequence, full parent block ID,
historical next-set hash, strict weighted quorum and transaction-data checks remain
mandatory. A peer signature or current roster cannot replace those checks.

authenticate_current_application_version checks a StateVersion's exact immutable
identity and recomputes EVE_APP_V1 from genesis, protocol, H, both roots and execution
hash. Its own latest certified header must be exactly H+1 and carry that commitment.
The returned private AuthenticatedApplicationAnchor exposes those committed fields
and native proof identity. An arbitrary external VerifiedNativeHeader cannot be
substituted. Height zero has no ApplicationCommitment and is rejected by this API.

StateVersion timestamp and content_digest are auxiliary inputs not separately
authenticated by EVE_APP_V1; they are deliberately absent from the anchor. No
complete-data validation or execution correctness follows from matching a root.
Importers still validate bounded full content, exact delta bases and reconstructed
roots; independent replay executes ordered authenticated transactions.

This first slice supports the existing classical development protocol only. Its
fixed profile/epoch comes from canonical genesis; native set hashes do not encode
EVM owners or key epochs. Dynamic owner/profile/key-epoch evolution, checkpoint
trust-period verification, replay and conflicting-history quarantine require their
actual subsequent implementation. There is no EVE checkpoint-import constructor
that pretends a downloaded anchor is trusted or fresh.

The baseline assumes less than one-third Byzantine voting power and strict
3*S > 2*T. These APIs do not provide PQ security, majority-attack immunity, fresh
head availability, durable storage or B4 acceptance by themselves. Private fields
prevent constructing verified results directly; local genesis selection remains an
explicit caller trust boundary. Production genesis remains unauthorized.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# EVE development finality and application anchoring

Canonical validator-owned pure verification consumed by role runtimes. This
component depends on native consensus verification, protocol configuration and
canonical logical state and the canonical deterministic execution component.
It contains no master authority, database, signer or live transport.

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
trust-period verification and conflicting-history quarantine require their
actual subsequent implementation. There is no EVE checkpoint-import constructor
that pretends a downloaded anchor is trusted or fresh.

The recovery capability replays bounded compact records from a locally initialized
genesis and a private verified parent. It derives execution context from certified
H, compares the complete replayed target/block, and anchors it through H+1.
Only fixed genesis owners/set/profile/epoch are supported; announced set changes
fail before publication. The exact immutable envelope remains attached to the
private transition. Public RAM publication and actual storage acknowledgement
are separate responsibilities. The [public applied service](../../../public/src/sync/applied/README.md)
integrates an explicitly limited empty-block capability; full B4 remains unfinished.

The codec uses canonical state payloads and native Prost messages, with bounded
preflight and exact reencoding. Its payload cap is 4,198,312 bytes under the
current opaque storage profile; some valid maximum-size blocks require later
versioned fragmented storage. No limit is raised or complete B4 acceptance claimed.

The [authenticated import capability](src/recovery/import/README.md) applies
bounded typed journals and authenticates reconstructed roots/header through H+1
without REVM. Its private imported state/transition cannot substitute for replay
capabilities. Both modes share one canonical history operation; imported fees are
not applied again. The exact-local-parent and auxiliary representation contract
remains explicit. Public wire, storage and RAM import integration is still required.

The baseline assumes less than one-third Byzantine voting power and strict
3*S > 2*T. These APIs do not provide PQ security, majority-attack immunity, fresh
head availability, durable storage or B4 acceptance by themselves. Private fields
prevent constructing verified results directly; local genesis selection remains an
explicit caller trust boundary. Production genesis remains unauthorized.

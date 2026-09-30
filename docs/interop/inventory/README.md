# INT0 interoperability inventory

INT0 freezes compile-tested public metadata and identifies real source-verifier
requirements. It does not implement authenticated source history, custody effects,
SDK signing, real external endpoints or deployment approval.

- [Version-one encoding and fixture identities](encoding-and-fixtures.md)
- [Exact source and tool pins](source-and-tool-pins.md)
- [Per-direction proof feasibility and blockers](source-proof-feasibility.md)
- [Acceptance outcomes and coverage](gate-outcomes.md)

Canonical source: `public/components/interop/` owns public route discovery and
admission metadata. Future deterministic bridge verification/accounting remains
validator-owned. External transport does not enter deterministic execution;
master has no source-finality, bridge approval or custody authority.

All four required directions are represented: Ethereum -> EVE, EVE -> Ethereum,
Solana -> EVE and EVE -> Solana. All remain VERIFIER_INCOMPLETE and
DISABLED_NOT_APPROVED. A metadata test cannot produce a VerifiedBridgeMessage;
no such constructor or fake finality-verifier implementation is exposed.

The type/codec/amount/registry tests cover foundational parts of T-I01/T-I02/T-I06.
Remaining endpoint/client/proof/SDK/incident/package gates remain NOT_IMPLEMENTED
or NOT_RUN. Ethereum/Solana cryptographic assumptions remain classical/external,
and EVE's required active paired profile is not yet enforced by consensus.

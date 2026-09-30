# SEC0 security inventory

This inventory records authorization and commitment boundaries at B0. It is not
an audit, an activated network profile or SECURITY_PROFILE_ACCEPTED. Runtime
coverage remains explicit in the linked tables; a working primitive does not
close missing consensus, account, recovery, client or custody authorization.

- [Authorization paths](authorization-paths.md)
- [Commitments and identity strength](commitments-and-identities.md)
- [Crypto sources and maintained reference](crypto-sources.md)
- [Adversary assumptions and expected outcomes](adversary-and-gate-outcomes.md)
- [External-chain source verification](../../interop/inventory/source-proof-feasibility.md)

The canonical ML-DSA/paired authorization source belongs to
`validator/components/authentication/`; public route metadata belongs to
`public/components/interop/`. Neither component implements validator finality,
trusted enrollment, production signing, a bridge proof or deployment approval.

Keep CLASSICAL_DEV, HYBRID_EXPERIMENTAL laboratory coverage and a future verified
profile separate. EVE/external-chain authentication assumptions do not change the
CometBFT less-than-one-third Byzantine baseline. Required 51% continuity remains
UNSATISFIED_BY_BASELINE.

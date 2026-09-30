# Per-direction source-proof feasibility

Status: source/API inspection and compile-tested B0 metadata. Every actual source
authentication/destination custody path below is NOT_IMPLEMENTED. No RPC
observation, elapsed wait, relayer signature or matching root is a verified claim.
No additional attestation committee/provider trust has been authorized.

| Required direction | Actual source authentication needed | Destination path / concrete remaining work | State |
|---|---|---|---|
| Ethereum -> EVE | Trusted beacon anchor, BLS sync-committee authentication/transitions, finalized branch, actual fork-specific execution-payload binding; successful custody receipt/event inclusion | EVE deterministic verifier and actual custody contract; bounded SSZ/MPT/BLS costs; execute upstream positive/negative light-client fixtures and real local consensus evidence | VERIFIER_INCOMPLETE; DISABLED_NOT_APPROVED |
| EVE -> Ethereum | EVE historical validator/profile transitions, paired authentication when activated, strict weighted quorum, execution header plus authenticated H+1 application commitment and custody inclusion | Real Solidity destination verifier/claim; ML-DSA/Ed25519 support/gas must be implemented and measured, not bypassed; protect replay/backing/upgrades | VERIFIER_INCOMPLETE; DISABLED_NOT_APPROVED |
| Solana -> EVE | Actual activated protocol and authenticated genesis/epoch ledger state; full checked replay or complete authenticated certificate/history binding where active, successful custody account/instruction effects | EVE source client with authenticated stake/key transitions and execution evidence; real Solana custody program; no fabricated receipt MPT or RPC-finalized proof | VERIFIER_INCOMPLETE; DISABLED_NOT_APPROVED |
| EVE -> Solana | Same EVE authenticated history/profile/H+1 custody binding; exact source economic sequence | Real destination program verifies paired source proof, correct PDA/account/mint/token-program/owner/authority, atomic consumption and value effects; proof staging/compute/size measured under active format | VERIFIER_INCOMPLETE; DISABLED_NOT_APPROVED |

## Ethereum source boundary

The [pinned Altair light-client specification](https://github.com/ethereum/consensus-specs/blob/5fa6edcca8ab4cf548653e6680b17b9d3e04d225/specs/altair/light-client/sync-protocol.md)
has authenticated committee/update processing, not a generic JSON-RPC receipt
wrapper. [Electra's pinned extension](https://github.com/ethereum/consensus-specs/blob/5fa6edcca8ab4cf548653e6680b17b9d3e04d225/specs/electra/light-client/sync-protocol.md)
changes generalized indices and execution-payload-header binding. v1.6.1 includes
Fulu specifications and inherits the applicable light-client extensions; there
is no fictitious `specs/fulu/light-client/sync-protocol.md` file to cite. INT2 must
bind the actual network fork schedule and run valid/stale/wrong-branch/committee
transition vectors. An optimistic update or `finalized` RPC label cannot release
finality-protected value. Source BLS/user/admin authority remains classical.

## Solana source boundary

The [pinned Agave certificate source](https://github.com/anza-xyz/agave/blob/825efd18292aff6ffcf9daa0f7612f21b3531a72/votor-messages/src/certificate.rs)
contains Alpenglow BLS certificate types, including slot-only slow finalization
and block-bound fast finalization. This is evidence of a real integration surface,
not evidence that every target cluster has activated the same feature/protocol.
Legacy ledger/vote replay remains relevant where selected by actual configuration.

The [actual `verify_certificate` API](https://github.com/anza-xyz/agave/blob/825efd18292aff6ffcf9daa0f7612f21b3531a72/bls-cert-verify/src/cert_verify.rs)
checks aggregate signature/bitmap/stake using caller-provided total stake and a
rank -> stake/proof-of-possession key map. It deliberately does not maintain the
ledger state. Therefore a standalone call with a relayer-supplied rank map is not
authenticated finality. INT2 must authenticate genesis/epoch rank/key/stake
changes, selected certificate type, relevant block binding and successful custody
effects under the actual feature set. A slot-only certificate alone does not
identify an arbitrary supplied state root or transaction. No RPC-only fallback
or automatically trusted signer committee may replace those missing checks.

Destination proof budgets must bind the actual transaction format and feature
activation. [Current Solana transaction documentation](https://solana.com/docs/core/transactions)
distinguishes legacy/V0 1232-byte and v1 4096-byte formats. A 3309-byte ML-DSA-65
signature does not fit legacy/V0; v1 may carry a signature alone but not an
unbounded multi-validator proof with keys/envelopes. Exact packet/account/compute
limits require the pinned local runtime test. If staging is needed, authenticate
route/author/content hash/chunk indices/length/expiry and prevent substitution or
early consumption. Never disable mandatory PQ verification to fit a packet.

## Prerequisites and failure classification

Actual EVE consensus finality, durable signing/enrollment and historical client
authentication are upstream B3/SEC1 dependencies. Shared atomic replay/backing and
the two-real-EVE fake-asset fixture belong to SEC2. Ethereum/Solana endpoint and
SDK work can proceed in INT1 independently of a blocked finality verifier.

The current blocker is missing implementation/integration evidence, not a proved
impossibility of external verification. Source anchors, active feature/fork
configuration and production custody/trust choices are still unavailable or
unauthorized. Keep these route-specific blockers visible through INT2/INT3.
Neither local fake-token application tests nor the source libraries above close
T-I03/T-I04/T-I05/T-I09/T-I10/T-I12 or end-to-end PQ security.

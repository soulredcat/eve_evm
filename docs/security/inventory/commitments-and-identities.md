# Commitments, prehashes and identity strength

Status: B0 coverage inventory. Required quantum properties depend on the use of
each hash and its input structure. No uniform whole-chain security category is
inferred from ML-DSA-65. Existing inner 256-bit commitments and 160-bit address
namespaces need explicit review before a protected-profile claim.

| Item | Format / purpose | Required review and remaining limitation |
|---|---|---|
| ML-DSA-65 external pure message | FIPS 204 `0x00`, context length, context, complete message; SHAKE internals | Bound domain/context and no prehash-interface confusion; official external/pure cases and cross-implementation checks |
| Paired authentication message | Existing versioned encoder binds genesis/network, purpose, identity/key epoch and payload | Enrollment, engine canonical-sign-byte relationship and activation are still absent; signature verification alone cannot trust enrollment |
| Ed25519 vote prehash/sign bytes | Actual selected CometBFT encoding | Freeze exact engine height/round/step/block/network fields; SHA-512 internally does not remove Ed25519's classical key assumption |
| Ethereum transaction signing | EIP-155 / typed-envelope Keccak-256 | Preserve EVM semantics; legacy secp256k1 authorization remains classical |
| EVM account address | Last 20 bytes of Keccak public-key hash; 160-bit routing identity | Routing/collision/targeted-substitution risk differs from signature strength; cannot claim 128-bit generic quantum preimage margin solely from a stronger outer signature |
| Validator/key identifiers | Actual engine key-ID derivation not yet frozen | Do not truncate paired identity to a collision-prone key selector; verify complete enrolled keys and epoch |
| Consensus block/header ID | Selected engine's canonical hashing | Includes the correct historical set, parent and application commitment; assess hash properties separately from signature authentication |
| EVM state/storage/receipt/transaction trie roots | Ethereum-compatible Keccak-256 MPT/trie commitments | A signature authenticates the committed bytes, not correct execution or available preimages; replay and root binding remain required |
| Code hash / CREATE / CREATE2 | Keccak-256 and 160-bit address derivation | Preserve deterministic semantics; supplemental commitment requires versioned migration, not silent format replacement |
| EVE execution/application commitment | Plan 14 versioned execution header/system root | H execution is linked by the authenticated H+1 application commitment; arbitrary root plus H certificate is insufficient |
| Public storage checksums/WAL/durable cursor | Recovery encoding and RocksDB durability | Corruption detection/durability is not source authentication; expose applied/durable/authenticated watermarks separately |
| Snapshot/delta/chunk hashes | Versioned chain/profile-bound recovery commitments | Authenticate anchor and replay tail; do not lose the only recovery copy; reject stale/foreign roots |
| Bridge economic/message IDs | Not implemented | Hash all namespace/genesis/route/deployment/asset/amount/recipient/sequence/profile fields; retries and alternate proof locators must not create new entitlement |
| Public route binding bytes | `eve-route-binding-v1` serialization, without an approval signature | Metadata only; future manifest hashing/authentication must include versioned implementation/anchor/upgrade policies and cannot trust this public field alone |
| Ethereum source evidence | Beacon SSZ/SHA-256, BLS sync committees, execution Keccak roots | Both fork-specific binding and inclusion needed; source assumptions are classical/external |
| Solana source evidence | Full 32-byte public keys, actual pinned ledger/BLS certificate format where active | Authenticate epoch rank/stake/key mapping and custody execution; no fabricated Ethereum receipt trie |
| Release/package/source identities | SHA-256 artifact digests plus future paired authorization | Digest pins identify bytes, not independent audit, execution validity, trustworthy genesis or license permission |

Supplemental wider commitments may protect a future native authorization path,
but wrapping a weak inner commitment in a wider digest does not restore lost
collision resistance. Checkpoint migration must authenticate key/profile changes
before classical authorization is assumed broken. Fresh nodes must not accept a
newly supplied classical anchor as PQ trust. Independent cryptographic review,
key-generation/backup/side-channel review and full T-P coverage remain open.

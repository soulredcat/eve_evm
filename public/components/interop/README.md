# Public interoperability metadata

Canonical owner: public admission and route discovery. The component validates
chain/genesis namespaces, full-width EVM and Solana addresses, origin assets,
bounded route metadata and exact integer decimal conversion. It exposes no
source-finality verifier, authenticated-message constructor, custody executor,
approval signature or external network operation.

Every B0 route remains disabled, including local fixtures and syntactically valid
metadata. Metadata fields supplied by a caller are assertions, not verified token
behavior or authenticated source consensus. Ethereum/EVE and Solana/EVE each
have two directional synthetic fixtures. They test encoding and admission, not
deployed endpoints, finality or value movement.

`src/chains/` owns network/address namespaces and canonical codecs;
`src/assets/` owns origin identity and bounded exact amounts;
`src/routes/` owns metadata validation, bounded discovery and immutable binding
serialization. Production bridge verification/accounting belongs to its later
canonical validator-owned component. Public metadata must never become a second
consensus or custody implementation.

The version-one binary contract and source-proof feasibility are recorded in
`docs/interop/inventory/`. Required golden vectors live inside this package's
`tests/fixtures/`. EVM canonical machine display is lower-case `0x` hex; checked
wallet display formats may be added separately. Solana display uses the maintained
`bs58` codec and preserves all 32 bytes.

Run `cargo test --locked -p eve-interop`. These foundational tests cover bounded
parts of T-I01/T-I02/T-I06. They cannot close the later complete interoperability
acceptance IDs. Workspace compilation does not satisfy standalone role packaging.

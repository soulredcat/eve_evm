<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator consensus acceptance

This test package launches four independent `eve-validator` foreground processes.
Each process owns its Rust application, remote signer, pinned native Comet engine,
and separate state, signing, replay, WAL and engine namespaces. No master process
or master acknowledgement participates in block production or transaction voting.
All nodes run on one Linux host; this does not demonstrate independent machine
failure domains, mainnet readiness, post-quantum authentication or a throughput
target.

The test fixtures create disposable keys in memory and write them only to
task-owned native Linux temporary directories with mode 0700 and seed files with
mode 0600. This remains necessary on hosts whose Windows-mounted workspace does
not preserve Unix permissions. Public genesis, public fixture configuration,
compiler output and raw diagnostics remain under ignored
`local-tests/b3-preparation/`. Temporary databases and private keys are removed
after the owned processes stop. A failed C06 run retains its task-owned private
Linux namespace for recovery diagnosis and writes only its path and binary
identities to ignored `retained-namespace.json`. Clear that disposable namespace
only after the retained recovery evidence has been reviewed. No fixture contains
a tracked private seed.

An exited owned child reports only fixed runtime failure categories and recognized
I/O kinds to the gate. The reader accepts a bounded private regular file for that
exact process identity, refuses symlinks and nonregular files, and redacts unknown
fields/codes. Raw panic output, key material, paths and detailed diagnostics remain
local. This summary supplies diagnosis, never execution or finality authority.

Engine discovery checks whether its owned validator has exited before waiting
for the unchanged discovery deadline. Startup failures carry only fixed CLI
stage/I/O categories stamped with that exact PID. The harness reads only a
bounded private regular stderr file, rejects links and permission/identity
mismatches, and redacts arbitrary text. A live validator that reaches the deadline
reports fixed child/image/argument observations plus available runtime categories.
The executable path and command checks remain mandatory; no failed startup is
treated as a passing process scenario.

Private validator CLI stdout/stderr stays in its owned native Linux node
namespace, where mode 0600 can be enforced even when the repository is mounted
from Windows. Discovery and readiness share the same exact-PID diagnostic path.
The failure summary is captured before namespace cleanup; retained failed C06
namespaces also retain those private logs. This does not relax file permissions.

The Cargo integration packet contains these actual process scenarios:

| Gate | Observable assertion |
| --- | --- |
| T-C01 | Four routing/signing identities, native proposer rotation, identical certified history and independently replayed complete state |
| T-C02 | Three of four active genesis voters progress; two halt after inflight drain; restored voters converge without quorum lowering |
| T-C03 | Real 2+2 P2P cut closes existing cross-group streams, rejects reconnects, halts both halves and heals one certified history |
| T-C04 | Actual native certificate rejects altered context, part-set identity, signatures, weight and historical roster |
| T-C05 | Selected malformed native proposal yields nil prevotes and later-round recovery; a protected EVM transaction that reverts still commits nonce, fees and failure receipt |
| T-C06 | Actual application/signer process crashes, durable signature-prefix preservation, restart progress and complete replay without duplicate transaction or fees |
| T-C07 | Genesis-pinned successful EVM transition receipts drive bounded rotation/leave/jail; H+1/H+2 native hashes and persisted H+3 callback metadata use applicable historical sets |
| T-C08 | Forged received roots cannot inherit native certificate authority or bypass complete-state validation |
| T-C09 | Execution commitment at H is bound by actual native header H+1; H0 uses the documented development content-digest anchor |
| T-C10 | Native RPC quota is occupied by bounded slow connections and overflow requests while consensus progresses without master |

Seventeen HTTP/native/submission regression cases execute once within the same integration
test: five RPC, three native decoding, three committed-submission and six observation cases.
They exercise actual local TCP framing, native JSON input and strict code/hash/height validation;
they do not replace the process scenarios. Test results and full-gate acceptance
are recorded by the integrator only after the complete frozen source is tested.

T-C07 submits its transition once through native CheckTx admission and separately
locates the exact transaction bytes in ordered native/application-covered blocks.
The native result must match the block height, transaction count and index with
explicit execution code zero. One existing 90-second action-progress budget starts
before admission and continues through verified history/replay and H+3 progress;
late success is rejected. Per-RPC three-second and connect one-second limits stay
unchanged. Every other scenario that submits a transaction uses the same one-shot
admission and observation with its own 90-second budget starting before admission.
A broadcast-commit request stays open across a failed three-second native proposal
round and so could outlast the per-RPC limit; the committed-submission cases keep
covering its shared code, hash and height decoders.
Block discovery and native result fields do not replace certificates or replay.

Native certificate verification uses the pinned canonical codec, RFC6962 hashes
and individual ZIP215 Ed25519 verifier. Applicable sets start from canonical
genesis and, for the temporary adapter, derive from independently replayed
successful authorized receipts. A peer-provided `/validators` response is compared
against that roster and never supplies its own trust. Native block header and
transaction-data hashes are checked. Full part-set identity is bound by the
certificate; this test does not independently reconstruct the block body's
part-set encoding. Nonempty unsupported native evidence fails decoding.

The peer proxies identify outbound connections through socket inodes owned by
the exact task-owned Comet `start --home` child, whose executable and start identity
are checked. Version and node-ID probes cannot become peer owners. Each native
node advertises its ingress proxy, preventing learned direct-address reconnects
from bypassing a partition. Existing streams are closed on a cut; no firewall,
unrelated process, external chain or host service is modified.

`AcceptanceValidatorTransitions.sol` is a disposable genesis contract compiled
with the provisioned pinned Solidity compiler for Shanghai. Its code trailer
binds authority, the `EVE_B3_ACCEPTANCE_V1` tag and SHA256 of the exact fixture file.
The contract enforces authorized ordered actions and emits an actual receipt
event. The feature-gated runtime adapter verifies those canonical receipts and
uses declared bounded updates. Rotation preserves the initial owner's 10,000
backed voting power. The fifth process is a future enrolled key for that owner.
Default runtime builds reject the marked genesis and fixture. This adapter is
temporary development acceptance material, not the B5 staking/penalty lifecycle.

The runtime's private authenticated native connection is the trusted baseline
for complete proposal/hash binding. ABCI ProcessProposal does not supply a full
native header, so execution approval cannot claim an independently reconstructed
header proof. The signer still executes against the actual canonical state and
requires durable anti-double-sign state before release. The separate certificate
and replay assertions verify the actual decided history.

Reproduce using the exact provisioned binaries and workspace lockfile:

```text
EVE_VALIDATOR_DEV_BINARY=<feature-built eve-validator>
EVE_VALIDATOR_NORMAL_BINARY=<separately preserved default eve-validator>
EVE_COMET=<pinned Comet executable>
EVE_COMET_SHA256=<verified executable SHA256>
EVE_SOLC_BINARY=<provisioned pinned Solidity executable>
cargo test --locked -p eve-validator-consensus-acceptance --test validator_consensus -- --test-threads=1
```

The B3 gate prepares these identities, runs strict feature Clippy and executes
the packet. Missing binaries, configuration or a mandatory assertion fails the
test; no process scenario is ignored or marked passed through a mock. SIGKILL
observations concern actual process exit and synced repository recovery. They
do not prove hardware power-loss durability, coherent-backup rollback detection,
cross-host clone fencing, majority immunity or automatic quorum recovery.

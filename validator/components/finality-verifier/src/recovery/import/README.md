<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Authenticated committed-state import

Canonical owner: validator finality verification. This pure operation authenticates
an imported EVM/system outcome against actual native H/H+1 history. It performs no
EVM transaction execution, network/storage I/O, publication, signing or fee update.
Public owns wire admission, actual leases, readiness and durable recovery material.

`AuthenticatedImportInput` is untrusted typed data: ordered `StateJournal`, complete
`BlockPayload`, finalized native H proof and complete certified H+1 lookahead data.
It is not a wire format and does not carry a peer-selected expected target digest.
Every component has borrowed admission bounds. Enclosing encoded payload, count,
fragmentation and durable-prefix limits remain separate caller responsibilities.

`journal.parent` must equal the receiver's exact local `StateVersion`, including
timestamp and complete-content digest. Another snapshot's auxiliary representation
may therefore be refused even when its committed roots match. No silent parent
rebind or relaxation of existing canonical replay equality is permitted.

Preparation verifies native certificates, exact ordered Data.Hash inputs, fixed
genesis validator set and enrolled proposer owners. Certified time/proposer/mixHash,
parent hash, derived base fee and active gas limit must match the execution header.
Unsupported profile/epoch/set activation fails closed under existing authority.

Ordered operations apply only to a private reserved candidate. Its complete roots,
header/hash, transaction/receipt roots, gas and bloom are checked through canonical
state operations. Existing hash history remains unchanged and exactly one correct
H/hash(header_H) entry is required. EVE_APP_V1 is recomputed and matched to the
actual certified H+1 application hash before an `ImportedTransition` is returned.

`ImportedState` and `ImportedTransition` have private fields. They are distinct from
`DevelopmentRecoveryState` and `VerifiedRecoveryTransition`; no conversion claims
independent execution. EVE_APP_V1 certifies both state roots and execution hash.
Unused hash-checked code and locally recomputed content_digest remain auxiliary
local representation, not independently certified exact values. Honest fixtures
can match every field of canonical replay; that is not a universal proof of exact
auxiliary representation from certificates alone.

The fixed classical BFT assumption and quorum threshold remain unchanged. Authenticated
import accepts the validator-certified outcome; it cannot independently detect an
execution error deliberately certified outside the honest-validator assumptions.
Independent canonical replay remains a separate oracle and operator mode. Fees
are already present in the delta and must never be applied a second time.

The conservative candidate reservation preflights logical growth before cloning.
The caller must hold real resource capacity through input/candidate/root/encoding
and retained-view lifetime. This is not an allocator/RSS guarantee. This component
alone does not complete B4, full fragmented persistence, peer recovery, snapshots,
standalone distribution, production security or throughput acceptance.

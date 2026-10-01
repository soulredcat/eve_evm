<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Classical development node assembly

This validator-owned domain assembles the actual canonical application, durable
signer, replay namespace, shared execution-approval registry, and pinned native
Comet process. `init_development_validator` initializes the same immutable local
namespaces without starting voting. Its result contains only the public native
node ID, chain/genesis identity, loopback addresses, zone metadata, and explicit
`CLASSICAL_DEV` profile. `run_development_validator` stays in the foreground until
SIGINT, SIGTERM, an owned-child failure, or a fatal authenticated channel error.

The root CLI delegates to these functions. Bootstrap uses canonical genesis and
state APIs; it contains no alternative execution implementation. The public JSON
specification is read once and decoded with the protocol-owned decoder. The
semantic JSON is also the native genesis application state. Native height zero
uses the canonical genesis content digest; positive heights use the normal EVE
application commitment. No quorum is lowered, and neither repository durability
nor readiness grants consensus finality or hybrid authentication.

The data directory must be a normal absolute Linux path under an existing real
parent. A new directory is created privately with mode 0700. An actual OS lease
excludes a second assembly, and a bounded immutable marker binds the canonical
genesis, enrolled public key, and engine digest. Existing nonempty unmarked data,
symlinked/insecure child namespaces, and changed identities are rejected. The
state, signer, replay, and native engine namespaces remain separate. Repository
APIs retain their own exact identity, synchronization, and recovery checks. The
node does not restore an old signing backup by assertion or recreate incomplete
repositories.

The native engine connects to the EVE-owned application Unix listener. EVE dials
the native engine's signer listener. Only real process-authenticated channel
proofs reach the private dispatchers. Each frame is surrounded by actual owned
child and live process checks. Process locks are released for socket I/O;
application and signer actor locks are held only for their respective dispatch.
The application actor services at most four native connections. Idle polling
does not consume frame prefixes or close healthy idle streams. A started frame
retains an absolute deadline. Transport-only signer disconnection permits bounded
authenticated redial; policy refusal is fatal. Redial never bypasses the retained
anti-double-sign ledger or missing execution/data approval.

Readiness is emitted once, after successfully written native Info/InitChain and
signer public-key responses plus an idle ping or successful signing response.
The native endpoint resets its idle ping timer on signing activity, so an active
network need not send an idle ping. A reopened positive canonical height does not
invent an InitChain callback. The output includes the current durable application
height/hash and sanitized signer height/round/step. It has no private key,
signature, signing bytes, or machine log paths. Its `consensus_finality` field is
false: authenticated certificates and the native voting protocol are independent
acceptance obligations. A single member of a four-validator set cannot advance
by itself.

Scoped worker threads belong to the foreground supervisor. An unexpected worker
return or unwind stops it. Close-only channel capabilities interrupt partial
reads during shutdown. Only the task-owned engine is stopped and waited; socket
cleanup checks the original application socket inode and preserves replaced
paths. Startup uses fresh process-specific socket names and never overwrites an
existing endpoint. SIGTERM and SIGINT do not create detached services.

Native signer connection deadlines are upstream operational limits, distinct
from the application frame deadlines. Concurrent launch must be exercised with
the actual verified artifact and reference build configuration; successful
single-node initialization does not establish four-validator acceptance. The
tracked runtime tests cover immutable namespace/lease guards, actual native
initialization, and real foreground handshakes plus owned-process shutdown. Full
B3 network, certificate, Byzantine fault, and lifecycle gates belong to the bulk
acceptance suite. Classical development and these process checks do not satisfy
the stronger SEC1 or hardware power-loss threat models.

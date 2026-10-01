<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Private proposal execution approval

Canonical owner: validator runtime. This module executes the supplied full proposal
against an immutable StateService parent, validates context and issues a private
non-nil-vote capability. Fields and production construction remain private. No peer
boolean, public caller flag, raw database handle or deserialized approval creates it.

The factory accepts only `VerifiedLocalEngineProposal`, created by private transport
after authenticating the task-owned pinned native engine connection. The exact ABCI
ProcessProposal request lacks a full consensus header and part-set commitment.
Native ValidateBlock and the authenticated local engine are therefore the explicitly
trusted baseline for request.hash/data/environment binding. This module does not
claim an independently reconstructed header proof, enclave or additional crypto.

Validate genesis/chain/profile/epoch, complete current parent and height, hash width,
timestamp, proposer binding and retained preceding consensus hash. Then call the
canonical execute_state_block operation, including actual envelope validation,
ordered execution, receipts, atomic rollback and 40/30/30 fees. Invalid transactions
reject; valid REVERT/out-of-gas remains valid included execution. No committed state
or durable marker advances during this speculative preparation.

The approval retains the complete request/data and prepared transition, parent
content identity and database sequence, runtime context and consensus block hash.
The same identical validated block may be used across rounds while its parent and
context remain applicable. Another hash/height/identity or stale parent cannot reuse
it. A restart cannot restore approval from a flag: retain/retrieve necessary data,
execute again through the authenticated request path or refuse new non-nil signing.

The signer records the complete native BlockID in canonical sign bytes. The ABCI
request does not supply its part-set root, so that first request's part-set linkage
remains a native-engine trust assumption; conflicting same-H/R/S part sets are
rejected durably. Independent full-header/certificate/replay verification stays in
the canonical consensus component and is not fabricated from this partial input.

Proposal signatures happen before ProcessProposal and cannot wait for this factory.
Already durable signature retries are separate from issuing a fresh approval.
Tests use an explicitly cfg(test)-only transport provenance fixture and actual EVM
execution; they are not substitutes for four real engines and authenticated Unix
transport acceptance. This capability grants neither finality nor custody authority.

## Resource and callback registry

InvalidExecution retains the canonical deterministic envelope/nonce/block-gas
failure. Unavailable retains local state/clone/resource failures or stale/fenced
context; those cannot be presented as execution-invalid peer blocks. Application
and transport must handle the typed distinction and fence signing on uncertain
durable application state.

One shared registry retains at most two full approvals and four original sealed
inputs, with a 16 MiB encoded retained-input limit. Full prepared-state images,
request/data copies, reconstruction scratch and allocator overhead require their
separate runtime reservations; this is not a total RSS or zero-copy claim.

Controlled replacement consumes a fresh non-Clone ProcessProposal token. The
pinned engine returns on LockedBlock before calling ProcessProposal and serializes
that callback under its consensus lifecycle; this is explicit trusted-native
unlocked evidence. FinalizeBlock provenance is a decision, not unlocked evidence,
and cannot authorize this replacement. No public boolean selects the privilege.

Keep the current full candidate and most recent relevant prevote candidate; retain
the latest non-nil precommit input conservatively. Actual returned enrolled native
signatures establish pins. Only unneeded speculative inputs may be dropped within
the fixed bounds. Protected capacity exhaustion returns Unavailable, never pruning
signing history, fabricating approval or weakening a test limit.

Retained tokens stay inside private wrappers and cannot be returned as owned fresh
callbacks. Reconstruction borrows their full data after dropping the cache lock,
executes again and supplies a separately reserved transient approval. It provides
no fresh unlocked replacement authority. Actual state-service publication clears
old-parent entries after synced commit. Restart loses this RAM registry and must
obtain an authenticated fresh native callback or retained authenticated data;
new non-nil signing remains refused until execution approval exists.

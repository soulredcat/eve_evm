<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Private validator signing orchestration

Canonical owner: validator runtime. This private module owns signer configuration,
cached H/R/S policy, replay, record validation and synchronous signature publication.
It is not a reusable public signing API, consensus algorithm or enrollment authority.
The public-owned opaque repository owns database access, exclusive local writing,
bounded history integrity and synced CAS. Runtime transport owns authenticated native
engine requests; the approval module owns actual canonical proposal execution.

## Identity and active profile

`SignerConfig` binds expected genesis, consensus chain, enrolled public key,
authentication requirement and key epoch. The supplied secret key must derive that
public key. Actual state identity/profile/epoch must match the expected context.
The runtime must obtain expected enrollment and historical membership from trusted
genesis and applicable-height history. A caller-created config is not enrollment.

The unextended native profile is CLASSICAL_DEV. Activated classical-AND-PQ startup
is refused; no post-finality wrapper or single native key alternative bypasses it.
Secret keys remain in memory and are absent from records, status and error output.
Native signature validation uses the canonical component's pinned ZIP215 verifier;
key-admission policy remains separate from native hash/signature semantics.

## Publication and retry

New signatures require the current complete state parent and next execution height.
Non-nil votes additionally need a private `ExecutionApproval` issued by executing
the complete authenticated local-engine proposal against the actual StateService.
Nil votes are explicit separate cases. Proposal signatures precede ProcessProposal
and use their own current-height/chain/HRS rule; they grant no vote approval.

Native H/R/S ordering is lexicographic: proposal=1, prevote=2, precommit=3. Lower
height/round/step or a conflicting message at the same H/R/S is rejected. Exact
and timestamp-only retries return the stored signature and original timestamp.
Already durable retries are resolved before the new-signature current-height
guard because application recovery may already have advanced. Such retries do
not create a new signature or claim fresh execution/data availability.

Before release, append and sync the complete canonical sign bytes, actual returned
64-byte signature, H/R/S, schema/profile and immutable identity/epoch in one opaque
CAS record. Cache publication follows successful storage acknowledgment. An append
failure fences the signer; reconciliation/reopen is required. History capacity
exhaustion never prunes signing records to continue voting.

On reopen, validate the complete bounded contiguous opaque history, canonical JSON
payload, identity/schema, native canonical message, individual signature and strict
H/R/S progression. Only the latest validated record is retained as the retry cache.
No key, seed or caller-supplied approval boolean is accepted as record metadata.

## Explicit recovery limits

Local RocksDB locking prevents simultaneous writers on the same namespace. It does
not fence a copied key/store at another path or host. A coherent older complete
backup cannot prove freshness from its own checksums. Runtime/operator fencing,
safe backup policy and trusted recovery continuity remain separate obligations.

Tests distinguish labelled simulated boundaries from actual child-process exit
after synced append. Neither proves hardware power-loss behavior, generalized
backup rollback detection, finality, quantum security or majority-attack immunity.
Full four-validator/runtime acceptance and standalone packaging require their own
integrated gates. Raw local artifacts stay under ignored local-tests/b3-preparation.

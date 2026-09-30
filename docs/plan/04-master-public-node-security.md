# 04 — Runtime trust boundaries

The threat model assumes an attacker may control a public node, a minority validator, a bootstrap peer, a snapshot source, or a master host. Network isolation reduces exposure; it does not make the master unreachable through every exploit chain or make authenticated payloads safe.

Master administration and databases have no public endpoints. Prefer master-initiated synchronization to allowlisted relays, separate management access, least privilege and independent credentials. Public and validator nodes use versioned object APIs, never filesystem mounts or database credentials.

A master must verify consensus provenance and commitment binding before publishing finalized data. A public node must verify the same provenance when downloading from master or peers. The source's identity is not sufficient proof that a state transition was valid. A master compromise cannot authorize arbitrary balances under an uncompromised validator quorum and correctly implemented verification.

Validator RPC, consensus signing and operating-system administration have separate privilege boundaries. Consensus signing must survive restart safely and prevent concurrent use of a key on multiple hosts. Public RPC workload must not starve signing or commit durability.

Bound message sizes before decoding, decompression and allocation. Apply connection quotas, request deadlines, replay protection, per-lane queue limits and overload shedding. Fuzz untrusted encodings and run fault injection in the task-owned devnet only.

Source publishing is not key management: no seeds, credentials, recovery secrets or private signing material belong in this repository. Code in this public repository is not made private by putting it in `master/`.

See [19](19-security-and-release-engineering.md) for attacks, defenses, key separation, signed releases and adversarial acceptance checks.

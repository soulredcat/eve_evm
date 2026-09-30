# 04 — Runtime trust boundaries

The threat model assumes an attacker may control a public node, a minority validator, a bootstrap peer, a snapshot source, or a master host. Network isolation reduces exposure; it does not make the master unreachable through every exploit chain or make authenticated payloads safe.

Master administration and databases have no public endpoints. Prefer master-initiated synchronization to allowlisted relays, separate management access, least privilege and independent credentials. Public and validator nodes use versioned object APIs, never filesystem mounts or database credentials.

Public nodes receive a logical synchronization endpoint or relay address rather than privileged master discovery or an internal database inventory. One protected master can supply many public replicas through bounded serving lanes. This reduces unnecessary exposure but does not guarantee master anonymity: routing, endpoint operators, traffic observation and software compromise can reveal infrastructure.

A master must verify consensus provenance and commitment binding before publishing finalized data. A public node must verify the same provenance when downloading from master or peers. The source's identity is not sufficient proof that a state transition was valid. A master compromise cannot authorize arbitrary balances under an uncompromised validator quorum and correctly implemented verification.

The same verification applies between masters. A second or later regional master is an independent follower with its own durable namespace; another master's authenticated transport identity does not authorize its root, validator set or claimed finalized height. Mutually synchronized masters exchange verified finalized history rather than merge writable chain states. Use single-writer fencing where processes share one mutable storage namespace, not a global writable-database lease across independent replicas.

Select synchronization sources in two stages. Enforce authenticated identity, network/genesis, protocol and active security-profile compatibility, valid historical finality/commitment proofs and required data availability first. Assess lag using authenticated heights and successful bounded retrieval, not an unverified status claim. Rank eligible sources by measured application-service latency, verified-data throughput and recent reliability. ICMP ping does not authenticate a source or prove usable history.

Keep one preferred logical endpoint with bounded fallback to another eligible endpoint or peer. Limit discovery, probe concurrency, retries and in-flight bytes; apply switching hysteresis and a cooldown to avoid repeated endpoint churn. Preserve the verified base and resume cursor across changes. Public transaction/live-block P2P paths and validator consensus must remain independent of the chosen master or bulk endpoint.

Validator RPC, consensus signing and operating-system administration have separate privilege boundaries. Consensus signing must survive restart safely and prevent concurrent use of a key on multiple hosts. Public RPC workload must not starve signing or commit durability.

Bound message sizes before decoding, decompression and allocation. Apply connection quotas, request deadlines, replay protection, per-lane queue limits and overload shedding. Fuzz untrusted encodings and run fault injection in the task-owned devnet only.

Public RAM working state has durable verified block/checkpoint recovery data by default. Isolate and bound its configured storage path and expose the actual durable height, disk pressure and recovery readiness. Do not advertise RAM or page-cache state as durable. This storage policy cannot relax validator signing durability, finality verification or safe last-copy retention.

Source publishing is not key management: no seeds, credentials, recovery secrets or private signing material belong in this repository. Code in this public repository is not made private by putting it in `master/`.

See [19](19-security-and-release-engineering.md) for attacks, defenses, key separation, signed releases and adversarial acceptance checks, and [32](32-regional-masters-and-public-persistence.md) for the regional endpoint and public-storage requirements. These remain implementation requirements, not completed defenses.

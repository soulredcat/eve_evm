# 31 — Mainnet performance target and separate application modules

Status: owner scope clarification recorded 2026-09-30. Documentation only; no performance result, funded reserve, custody deployment or working application is claimed.

## MB01 — Latest instruction takes precedence

The owner withdrew liquidity and automatic DEX price stabilization from the current EVE core implementation goal: these will be a separately developed module later. Do not implement the earlier proposed reserve-funded stabilization system, add it to the mandatory core queue, or make core acceptance wait for it.

This instruction narrows application scope. It does not cancel EVM execution, validator finality, master synchronization, staking/rewards, security, Ethereum/Solana bridge interoperability, or the user's folder/function-file policy.

## MB02 — Core versus separate future module

| Area | Current treatment |
|---|---|
| EVM, public/validator/master roles, storage, networking and RPC | Mandatory core work under existing specifications. |
| Staking, fee split, PQ/security and bridge/interoperability | Existing mandatory tracks remain in scope. |
| Liquidity provision, production DEX pools, automated market-making or price stabilization | Deferred separate application module; not part of the current core goal. |
| Reserve-funded intervention, automatic buyback/mint/sell and pool-price controllers | Do not build or activate in this goal. |
| Solidity swap/AMM fixtures and contention benchmarks | Retain as disposable compatibility/correctness/performance tests, not a production DEX commitment. |
| Earlier reserve/backing and multi-admin lock requirements | Retain as a separate economic/vault proposal under MB05; no implied trading authority. |

Do not create placeholder liquidity runtimes, privileged stabilizer hooks, unused deployment scripts, price oracles or reserve spenders merely to anticipate the future module. Ordinary reusable EVM/account/transaction APIs are enough until a separately authorized module has an actual interface requirement.

A future module should use versioned public interfaces and normal authenticated transactions or explicitly approved system interfaces. It must not require direct master database access or change consensus rules by making an off-chain price call. Its failure, pause or absence must not prevent unrelated core transactions from finalizing.

## MB03 — Proven 1M TPS is a mainnet release requirement

The owner's launch target is 1,000,000 aggregate finalized user transactions per second, demonstrated rather than projected. MAINNET_READY must require SCALE_TARGET_VERIFIED for the release candidate using the active required security profile and production-representative topology, configuration, workload, persistence and network conditions. A working devnet below the target is an intermediate result, not authorization to lower this launch requirement.

Use plans 08, 20, 21 and security/interop specifications for sustained/soak runs, mixed-EVM workload, bounded backlog, latency, correct execution, data availability and recovery. Count each economic user transaction once; do not add replicas, bridge retries, internal calls or batch roots to manufacture throughput. Publish single-hot-pool and cross-domain results separately from aggregate low-contention capacity.

A production-representative prelaunch benchmark is not proof that live mainnet already processed that volume. Label prelaunch capacity, offered load, actual finalized load and later live-mainnet measurements distinctly. Failed or unavailable measurements retain TARGET_UNMET or BLOCKED_INFRA; mainnet deployment and spending still require owner authorization.

## MB04 — Future 100M and 1B TPS extensibility

Preserve a measured development path toward 100,000,000 and potentially 1,000,000,000 aggregate finalized TPS as technology and infrastructure improve. These are future research/capacity goals, not current guarantees, automatic consequences of more nodes or additional launch gates beyond the accepted 1M requirement.

Avoid arbitrary implementation constants that permanently cap the whole architecture at 1M. Keep replaceable, versioned execution/consensus/storage/sync interfaces and checked counters/resource units. Retain finite, enforceable active-network gas/byte/queue limits: extensibility is not permission for unbounded resources or unilateral local limit changes.

Any later scale-out, sharding, proof system or consensus replacement must preserve the declared EVM, finality, availability, security and recovery semantics or explicitly version and approve changes. Benchmark each step with the real profile and workload. Current architecture need not be called capable of 100M/1B before that evidence exists.

## MB05 — Reserve and multi-admin clarification retained separately

The owner previously proposed USD 100M of backing associated with 100M base tokens, possible gold/other collateral later, and multiple administrator confirmations for unlocking. Record this proposal without claiming funds exist, fixing a production ticker/supply/genesis, establishing redemption rights, or promising a DEX price.

The liquidity deferral does not grant any component permission to spend those reserves automatically. Keep future reserve/vault requirements separate from liquidity policy and from bridge-user custody. Do not pledge the same assets to several obligations or treat a dashboard entry as verified collateral.

For the recorded on-chain-lock direction, authorization must ultimately be enforced by consensus-checked vault/account rules. Master may retain the finalized data but must not gain unilateral release, minting or consensus authority. The number and independence of admins, k-of-n threshold, timelock, key rotation/recovery, active signature profile and exact approved assets/amounts/destinations must be specified before implementation or deployment. No arbitrary mainnet threshold is selected here.

Real custody, off-chain asset evidence, asset mapping, issuance and withdrawal/redemption policy remain owner-gated. A future detailed vault specification may be developed separately; this scope clarification does not add reserve trading, automated stabilization or live fund movement to B0–B11.

## MB06 — Goal and review enforcement

Read this document with the decision register before selecting work. Keep the existing core, security and interop queues; do not invent a liquidity/stabilization bulk. Record a future request in a separately scoped plan instead of importing it into consensus by default.

Review each bulk for unwanted liquidity dependencies or reserve-spend privileges. Retain AMM fixture tests and bridge lock/mint/burn/unlock accounting: neither is the canceled price-stabilizer implementation. If actual runtime code later contains a now-deferred component, inspect ownership and preserve unrelated work; use a reviewed isolated removal/refactor with regression evidence rather than deleting a broad folder blindly.

No runtime test was performed by this documentation revision. Structural, functional and performance completion still requires the existing implementation gates, and the 200/400/600 file policy is unchanged.

<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Independent state-root vectors v1

These public vectors use EthereumJS MPT/RLP/util 10.1.3, source commit
`29f404f305be4473907d4aa4f7ece85633bb73da`. The implementation is separate from
Rust Alloy and EVE state/execution/storage code. The reference packages are
MPL-2.0; dependency source is not copied into this repository and this notice
does not choose an EVE source license.

Primary package identities:

| Package | Publisher integrity |
|---|---|
| [@ethereumjs/mpt 10.1.3](https://registry.npmjs.org/@ethereumjs/mpt/10.1.3) | `sha512-pN43KoH8V0afpIltMFJf0tSSpX9Xssp9LGAyamnYc93oe0r8bRafeizPWR9aDsS+WXSg/IWQscoZ3xTybQSYoA==` |
| [@ethereumjs/rlp 10.1.3](https://registry.npmjs.org/@ethereumjs/rlp/10.1.3) | `sha512-d795YWkN6O8EJh/PgJ2zZKCdBMl8OlqkuVQESoidH666gaFtUuJ01jDtiuqfyOqTNcTdGhHi8LkQv1WEZzvZZw==` |
| [@ethereumjs/util 10.1.3](https://registry.npmjs.org/@ethereumjs/util/10.1.3) | `sha512-PhezOVe4DFaC+QLIsnK/tiOaXjpjS2F9AH4Ez0EfWevFw9NWNSg95l7I0dWuHF0B7v9odm2Ijqwo0EnARXtUmA==` |

Generation used the B0 pinned Node 24.21.0/npm 11.19.0 and a private task-local
lockfile. Install scripts were disabled. The local-only manifest SHA256 is
`1b1e69e9777b4f8d7a743d88b7e7574a8248803364f08ca2a0b3ec35694e331f`;
lock SHA256 `05491d10e466b77a0df6ba54b2bb6301f5aa6c41ec7be221c97daa7aa1b693ad`.
Raw generator, output and dependency files remain ignored under
`local-tests/b1-preparation/oracle/`; the literal test inputs and expected bytes
needed by the acceptance tests are versioned here.

Reproduction follows each JSON file's literal account and fee data:

1. Create account and storage tries with `createMPT({ useKeyHashing: true })`.
2. Encode each slot key as 32-byte big endian. Insert only nonzero slot values,
   encoded as minimally represented unsigned RLP integers.
3. Encode an account as RLP `[nonce, balance, storageRoot, keccak256(rawCode)]`
   and insert under its raw 20-byte address. Account and storage keys are hashed
   exactly once by the secure trie.
4. For the system trie, use `useKeyHashing: false`. Its key is already
   `keccak256(rlp([utf8("fee"), utf8("pool")]))`; its value is
   `rlp([1, utf8("fee"), utf8("pool"), [burned, node_pool, validator_pool]])`.
5. Compare account RLP, storage/code hashes, system key/value and both roots to
   the versioned literal outputs. Delete an account by omitting its leaf; a zero
   slot creates no storage leaf. Explicit account presence remains meaningful.

The runtime `606360005500` is `PUSH1 99; PUSH1 0; SSTORE; STOP`. The execution
fixture starts with slots 0=7 and 1=11. Shanghai cold nonzero-to-nonzero SSTORE
costs 5000, the two pushes cost 6, and the empty-data transaction intrinsic gas is
21000: 26006 total. At gas price 2,000,000,000 the exact 40/30/30 allocation is
recorded in execution.json. These gas/fee inputs were specified independently;
the Node tool computes their trie representation and does not execute REVM.

Execution transaction bytes are deterministic EIP-155 legacy encoding for nonce
0, gas limit 100000, chain 31337, value/data empty and the stated contract. The
independent signer is locked `@noble/curves` 2.4.0 with prehash disabled over the
Keccak signing digest and RFC6979/low-S defaults. Receipt bytes are status 1,
cumulative gas 26006, zero bloom and no logs. Ordered transaction/receipt trie
keys are canonical RLP indices, without secure-key hashing. Tests compare the
literal envelope bytes, hashes and both ordered roots as well as state.

The public repeated-byte test key used only to identify the execution fixture
sender is deliberately unsafe and isolated to acceptance tests. It is not a
runtime default, faucet key or production signing identity. No funds, chain,
finality proof, authenticated snapshot or power-loss result is represented here.

`development_genesis_accounts.json` independently freezes the four 90000-unit
owner balances and the aggregate 40000-unit f100 escrow, preserving 400000 funded
units at 18 decimals. f101/f102 are absent before the first fee credit. This file
checks only the EVM allocation root; actual genesis additionally initializes
validator and parameter system records through the canonical development builder.

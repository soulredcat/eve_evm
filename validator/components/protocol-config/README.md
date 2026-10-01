<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Validator-owned development protocol contract

Canonical owner: `validator/components/protocol-config/`. This component owns
immutable development genesis, EVE consensus-bound record formats, the
Ethereum-facing execution header mapping, native ABI/metering and security-profile
compatibility. Public readiness/routing/resource policy belongs to
`public/components/node-policy/`; no public/private master orchestration lives here.
It depends on maintained Alloy encoding/header types, Ed25519 key validation, and
the pinned CometBFT authentication capability guard. It has no master dependency.

Read plans [13](../../../docs/plan/13-transaction-and-gas-spec.md),
[14](../../../docs/plan/14-block-and-state-commitment-spec.md),
[16](../../../docs/plan/16-genesis-upgrade-and-recovery.md),
[17](../../../docs/plan/17-validator-lifecycle-and-rewards.md),
[25](../../../docs/plan/25-folder-hierarchy-and-file-function-policy.md) and
[27](../../../docs/plan/27-post-quantum-cryptography-and-migration.md) before changes.

## Development genesis v1

`LaunchMode::Production` is rejected. The supported immutable development profile
is `eve-local-v1`, EVM chain 31337, schema/protocol 1, Shanghai, CLASSICAL_DEV and
four distinct equal-power Ed25519 validators. Enrollment requires canonical
nonidentity prime-order points through the existing pinned Dalek API; generic
native ZIP215 verification remains separate. See D44 and its preserved vectors.
Owners/accounts/keys are unique; declared self-bonds are funded by owner allocations.
Reserved f100/f101/f102 user allocations and user code there are rejected.

The encoder subtracts each self-bond from its funded owner's spendable balance
and puts the sum in f100 staking custody. Total supply remains the original sum of
funded allocations. f101/node and f102/validator fee escrows start at zero. The
fixture funds four owners with 100,000 development tokens each, escrows 10,000
each, and preserves 400,000 total; these are unsafe test identities, not live keys.

Canonical genesis RLP field order is:

```text
[EVE_GENESIS_V1, schema, protocol, name, EVM_chain_id, initial_timestamp,
 profile_tag, consensus_parameters, execution_parameters, economics,
 reserved_addresses, native_gas_schedule, sorted_spendable_accounts,
 sorted_validators, ordered_upgrades, staking_escrow_balance, funded_supply]
```

Account tuples are `[address, spendable_balance, nonce, code]`, sorted by address.
Validator tuples are `[owner, Ed25519_key, self_bond, power]`, sorted by key bytes.
Power is floor(self-bond / 10^18), positive, equal in this four-validator profile,
and total power cannot exceed the engine's `i64::MAX / 8` bound. Account/code and
upgrade counts/bytes are bounded. Upgrade tuples bind activation height, version,
profile, code digest and migration ID; order/version must strictly increase.
Unsupported hybrid activation is rejected. Upgrade execution remains B8 work.

Consensus parameters bind 30M gas, 4 MiB complete block bytes, 1000-block/24-hour
evidence windows, 24-hour checkpoint trust and 10k-block/24-hour retention minima.
Execution parameters bind Shanghai, 128 KiB transactions, 1 gwei initial base fee
and floor 1. Economics freezes plan 17's development values: 40/30/30 fees with
validator split dust, 1000-block epochs, max 64 validators, 10k/100-token self-bonds,
10%/20% commission, seven days AND 2000-block unbonding, max 32 tasks/block, 80%
availability/participation, 5% proven double-sign penalty, and zero default issuance.
Node service financial slashing is disabled. Mainnet economics and supply remain
owner decisions; these development constants grant no production authorization.

The genesis digest hashes these canonical bytes and contains no recursive app hash.
Execution `extraData` binds protocol `u32_be` plus the first 28 genesis-digest bytes.
The maintained Alloy Header encodes exactly Shanghai's 17 fields, with canonical
empty ommers/withdrawals, zero difficulty/nonce, and no later-fork fields. Timestamp
seconds are nondecreasing agreed inputs; PREVRANDAO equals the preceding consensus
hash, matching the current execution adapter, without an unbiased-randomness claim.

## Canonical system formats

The application commitment follows plan 14 exactly:
`keccak256(rlp([EVE_APP_V1, genesis, protocol, H, EVM_root, system_root, execution_hash]))`.
Root/hash domains have distinct wrappers. Construction/hashing authenticates no
certificate, peer or execution result.

System keys are `keccak256(rlp([namespace_bytes, logical_key]))`; logical keys are
1..128 bytes. Namespaces are `validator`, `fee`, `reward`, `parameter`, `task`,
`evidence`, `upgrade`. A v1 leaf value is
`[1, namespace_bytes, logical_key, payload_tuple]`, with these payloads:

| Namespace | Ordered payload fields |
|---|---|
| validator | owner,32-byte key,power,activation,optional removal,key epoch |
| fee | cumulative burned,node pool,validator pool |
| reward | owner,role,liability,reward index |
| parameter | parameter name,opaque bounded value |
| task | epoch,task ID,node,content commitment,request nonce,deadline height,work units,consumed |
| evidence | evidence ID,offense height,applied |
| upgrade | old version,new version,activation,code digest,migration ID |

Integers are minimally encoded unsigned RLP. Boolean false/true is integer 0/1;
absence is `[0]`, presence `[1,value]`, and empty bytes are the empty RLP string.
Delta tuples are `[1,namespace,key,0]` for deletion or
`[1,namespace,key,1,record]` for replacement. Namespace/key substitutions fail.
Export chunks are `[EVE_SYSTEM_EXPORT_V1,1,[[trie_key,record],...]]`, sorted by
canonical trie-key bytes, with duplicate rejection and at most 1024 records/chunk.
`validate_canonical_record_bytes` compares bytes after caller typed decoding; it
is not a raw transport decoder or a provenance verifier. Business transitions,
trie persistence, replay and reward conservation remain B1/B5 work.

## Native ABI and gas v1

`NATIVE_FUNCTIONS_V1` freezes ten mutations plus six read selectors; the golden
fixture freezes every selector and ten event topics. Development registration
uses transaction value for the self-bond; `delegate(address,uint256)` requires
matching value; the other mutation/read calls are nonpayable. The owner is the
recovered EVM sender, never an authority taken from untrusted calldata.
Key bytes are a canonical `[1,algorithm=1,key_epoch,Ed25519_key]` envelope;
possession proof bytes are `[1,64-byte_signature]`. Node registration's bytes32
is its raw Ed25519 public key with initial epoch 0. The signed possession message
binds domain,genesis,protocol/profile,chain,owner,role(1 node/2 validator),nonce,
key epoch and full key. B5 must decode/verify it and journal custody atomically.
Read return contracts are bytes for registry/task records, uint64 activation,
and uint256 bonded/unbonding/claimable amounts. Mutations return no ABI data.

Events index owner/task/evidence IDs and applicable peer/key identities; their
remaining numeric arguments are data. Signatures/topics are fixed in `native/abi.rs`.
This component provides codecs/constants, not callable native handlers. B5 must
implement selector dispatch, exact argument/proof bounds, authorization and revert
behavior; no empty-code success may substitute for a required system operation.

Native execution metering is an additional deterministic surcharge above ordinary
EVM transaction intrinsic gas; calldata/proof costs cover native parsing. Values:
dispatch 5000, zero/nonzero calldata 4/16, proof bytes 16, classical verify 3000,
ML-DSA-65 verify 100000, storage read 2100/new write 20000/existing write 5000.
Limits: 128 KiB calldata, 64 KiB proofs, 64 signature checks, 256 storage operations.
Handlers must derive counts from actual decoded work, not trust caller totals.
Worked classical registration costs 80016; adding one ML-DSA check and 3309 proof
bytes costs 232448. These conservative dev prices are not production benchmarks,
and calculating a PQ cost does not enable unsupported hybrid authorization.

`validate_network_profile` matches genesis/name/chain/protocol/profile/activation/
key epoch to caller-trusted applicable-height expectations. Unknown tags reject;
hybrid/PQ profiles call the native engine guard and reject even if peers agree.
Binding structs are data, not authenticated capabilities; SEC1/B4 must establish
history/proof provenance. No classical-only fallback enables an activated profile.

## Verification

`cargo test -p eve-protocol-config --locked` covers positive/negative configuration,
golden bytes/hashes, insertion-order invariance, encoding tags, ABI/topics/gas,
header shape and fail-closed profiles. The golden JSON freezes project encodings
and maintained Alloy outputs; it is not an independent Ethereum corpus or a
security proof. Hand-specified small RLP vectors also exercise integer/tag rules.
Run strict Clippy, format and the integrated structure/B0 gates before integration.

## B2 shared development input and base fee

`genesis::input::decode_development_spec` is a pure bounded JSON decoder. It rejects
input above 1 MiB, unknown/duplicate fields, unsupported profile and noncanonical
32-byte lowercase 0x public keys, then invokes the existing development genesis
validator and frozen economics. The canonical genesis identity is preserved.
Runtime file/path/private-credential/mode guards remain with public/master owners;
this decoder does not load signing material or authorize production genesis.

`headers::derive_next_base_fee` derives the next development fee from the active
parent gas use/limit: elasticity 2, denominator 8, minimum rise of 1 and fee floor 1.
It checks exact wide arithmetic overflow before invoking maintained EIP-1559 logic.
Missing/zero fee, invalid parent gas and overflow fail explicitly. Copying parent
basefee unchanged is not an accepted block-construction rule.

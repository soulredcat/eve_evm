// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { encodeAbiParameters, hexToBytes, keccak256, parseAbiParameters, toHex, type Hex } from "viem";
import { verifyMerkleProof } from "@ethereumjs/mpt";
import type { AcceptanceContext, TokenFixture } from "../types/acceptance_types.js";
import { rpcRequest } from "../transport/rpc_request.js";
import { verifyAccountProof, type AccountProofResponse } from "../proofs/verify_account_proof.js";
import { verifyStorageProof } from "../proofs/verify_storage_proof.js";

export async function verifyProofs(context: AcceptanceContext, fixture: TokenFixture): Promise<void> {
  const owner = context.accounts[4];
  assert.ok(owner);
  const slot = keccak256(encodeAbiParameters(parseAbiParameters("address, uint256"), [owner.address, 2n]));
  const nonexistent = "0x000000000000000000000000000000000000dEaD" as const;
  let previous: AccountProofResponse | undefined;
  for (const node of context.nodes) {
    const block = await rpcRequest<{ number: Hex; stateRoot: Hex }>(node.httpUrl, "eth_getBlockByNumber", ["latest", false]);
    const proof = await rpcRequest<AccountProofResponse>(node.httpUrl, "eth_getProof", [fixture.tokenA, [slot, toHex(999n, { size: 32 })], block.number]);
    await verifyAccountProof(block.stateRoot, fixture.tokenA, proof);
    assert.equal(BigInt(proof.storageProof[0]!.value), 898_983n);
    assert.equal(BigInt(proof.storageProof[1]!.value), 0n);
    const absent = await rpcRequest<AccountProofResponse>(node.httpUrl, "eth_getProof", [nonexistent, [toHex(0n, { size: 32 })], block.number]);
    await verifyAccountProof(block.stateRoot, nonexistent, absent);
    assert.equal(BigInt(absent.balance), 0n);
    await assert.rejects(() => verifyStorageProof(absent.storageHash, { key: "0x0", value: "0x1", proof: [] }));
    await assert.rejects(() => verifyStorageProof(absent.storageHash, { key: "0x0", value: "0x0", proof: ["0x00"] }));
    await assert.rejects(() => verifyStorageProof(proof.storageHash, { key: slot, value: "0x0", proof: [] }));
    const damaged = proof.accountProof.map((item) => hexToBytes(item));
    assert.ok(damaged[0] && damaged[0].length > 0);
    damaged[0][0] = damaged[0][0]! ^ 1;
    await assert.rejects(() => verifyMerkleProof(hexToBytes(fixture.tokenA), damaged, { root: hexToBytes(block.stateRoot), useKeyHashing: true }));
    await assert.rejects(() => verifyMerkleProof(hexToBytes(fixture.tokenA), proof.accountProof.map((item) => hexToBytes(item)), {
      root: hexToBytes(`0x${"42".repeat(32)}`), useKeyHashing: true,
    }));
    const wrongKeyValue = await verifyMerkleProof(hexToBytes(nonexistent), proof.accountProof.map((item) => hexToBytes(item)), {
      root: hexToBytes(block.stateRoot), useKeyHashing: true,
    }).catch(() => null);
    assert.equal(wrongKeyValue, null, "The queried account leaf must not authenticate another address");
    if (previous !== undefined) assert.deepEqual(proof, previous, "Independent RPC processes must return the same captured proof");
    previous = proof;
  }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { verifyMerkleProof } from "@ethereumjs/mpt";
import { RLP } from "@ethereumjs/rlp";
import { bytesToHex, hexToBytes, keccak256, type Hex } from "viem";

export async function verifyStorageProof(root: Hex, slot: { key: Hex; value: Hex; proof: Hex[] }): Promise<void> {
  const emptyRoot = keccak256(RLP.encode(new Uint8Array()));
  if (root === emptyRoot) {
    // EthereumJS 10.1.3 rejects an empty witness through its missing-node path.
    // The canonical empty trie root independently proves every slot absent.
    const canonicalEmptyNode = bytesToHex(RLP.encode(new Uint8Array()));
    assert.ok(slot.proof.length === 0 || (slot.proof.length === 1 && slot.proof[0] === canonicalEmptyNode),
      "An empty trie accepts only no witness or its exact canonical RLP empty-root node");
    assert.equal(BigInt(slot.value), 0n, "An empty trie cannot authenticate a nonzero storage value");
    return;
  }
  const key = hexToBytes(`0x${BigInt(slot.key).toString(16).padStart(64, "0")}`);
  const stored = await verifyMerkleProof(key, slot.proof.map((item) => hexToBytes(item)), {
    root: hexToBytes(root), useKeyHashing: true,
  });
  if (stored === null) assert.equal(BigInt(slot.value), 0n);
  else {
    const decoded = RLP.decode(stored);
    assert.ok(decoded instanceof Uint8Array);
    assert.equal(decoded.length === 0 ? 0n : BigInt(bytesToHex(decoded)), BigInt(slot.value));
  }
}

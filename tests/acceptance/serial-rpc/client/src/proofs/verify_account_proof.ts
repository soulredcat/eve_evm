// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { verifyMerkleProof } from "@ethereumjs/mpt";
import { RLP } from "@ethereumjs/rlp";
import { bytesToHex, hexToBytes, type Address, type Hex } from "viem";
import { verifyStorageProof } from "./verify_storage_proof.js";

export interface AccountProofResponse {
  address: Address;
  balance: Hex;
  nonce: Hex;
  codeHash: Hex;
  storageHash: Hex;
  accountProof: Hex[];
  storageProof: { key: Hex; value: Hex; proof: Hex[] }[];
}

export async function verifyAccountProof(root: Hex, address: Address, proof: AccountProofResponse): Promise<void> {
  assert.equal(proof.address.toLowerCase(), address.toLowerCase());
  const value = await verifyMerkleProof(hexToBytes(address), proof.accountProof.map((item) => hexToBytes(item)), {
    root: hexToBytes(root), useKeyHashing: true,
  });
  if (value === null) {
    assert.equal(BigInt(proof.balance), 0n);
    assert.equal(BigInt(proof.nonce), 0n);
    assert.equal(proof.codeHash, "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
    assert.equal(proof.storageHash, "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");
  } else {
    const account = RLP.decode(value);
    assert.ok(Array.isArray(account) && account.length === 4);
    const fields = account as Uint8Array[];
    const nonce = fields[0];
    const balance = fields[1];
    const storage = fields[2];
    const code = fields[3];
    assert.ok(nonce && balance && storage && code);
    assert.equal(nonce.length === 0 ? 0n : BigInt(bytesToHex(nonce)), BigInt(proof.nonce));
    assert.equal(balance.length === 0 ? 0n : BigInt(bytesToHex(balance)), BigInt(proof.balance));
    assert.equal(bytesToHex(storage), proof.storageHash);
    assert.equal(bytesToHex(code), proof.codeHash);
  }
  for (const slot of proof.storageProof) {
    await verifyStorageProof(proof.storageHash, slot);
  }
}

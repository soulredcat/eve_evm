// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { RLP } from "@ethereumjs/rlp";
import { hexToBytes, keccak256, type Hex } from "viem";
import { encodeQuantity } from "./encode_quantity.js";

export function verifyHeaderHash(block: Record<string, unknown>): void {
  const data = (name: string, size?: number): Uint8Array => {
    const value = block[name];
    assert.equal(typeof value, "string", `Missing header field ${name}`);
    assert.match(value as string, /^0x(?:[0-9a-f]{2})*$/, `${name} must be even-length hex data`);
    const bytes = hexToBytes(value as Hex);
    if (size !== undefined) assert.equal(bytes.length, size, name);
    return bytes;
  };
  const encoded = RLP.encode([
    data("parentHash", 32), data("sha3Uncles", 32), data("miner", 20), data("stateRoot", 32),
    data("transactionsRoot", 32), data("receiptsRoot", 32), data("logsBloom", 256),
    encodeQuantity(block.difficulty), encodeQuantity(block.number), encodeQuantity(block.gasLimit),
    encodeQuantity(block.gasUsed), encodeQuantity(block.timestamp), data("extraData", 32),
    data("mixHash", 32), data("nonce", 8), encodeQuantity(block.baseFeePerGas), data("withdrawalsRoot", 32),
  ]);
  assert.equal(keccak256(encoded), block.hash, "RPC execution hash must be the exact canonical 17-field Shanghai header hash");
  assert.equal(BigInt(block.difficulty as string), 0n);
  assert.equal(block.nonce, "0x0000000000000000");
  assert.equal(BigInt(block.gasLimit as string), 30_000_000n, "Live RPC profile must retain its declared gas limit");
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { RLP } from "@ethereumjs/rlp";
import { concatBytes, hexToBytes, type Hex } from "viem";
import { encodeQuantity } from "./encode_quantity.js";

export function encodeRpcReceipt(receipt: Record<string, unknown>): Uint8Array {
  const logs = receipt.logs as { address: Hex; topics: Hex[]; data: Hex }[];
  assert.ok(Array.isArray(logs));
  const payload = RLP.encode([
    encodeQuantity(receipt.status), encodeQuantity(receipt.cumulativeGasUsed),
    hexToBytes(receipt.logsBloom as Hex),
    logs.map((log) => [hexToBytes(log.address), log.topics.map((topic) => hexToBytes(topic)), hexToBytes(log.data)]),
  ]);
  const transactionType = Number(BigInt(receipt.type as Hex));
  assert.ok(transactionType === 0 || transactionType === 1 || transactionType === 2);
  return transactionType === 0 ? payload : concatBytes([Uint8Array.of(transactionType), payload]);
}

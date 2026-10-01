// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";

export function encodeQuantity(value: unknown): Uint8Array {
  assert.equal(typeof value, "string");
  const quantity = value as string;
  assert.match(quantity, /^0x(?:0|[1-9a-f][0-9a-f]*)$/, "RPC quantities must be minimal lowercase hexadecimal");
  const integer = BigInt(quantity);
  if (integer === 0n) return new Uint8Array();
  const hex = integer.toString(16);
  return Uint8Array.from(Buffer.from(hex.length % 2 === 0 ? hex : "0" + hex, "hex"));
}

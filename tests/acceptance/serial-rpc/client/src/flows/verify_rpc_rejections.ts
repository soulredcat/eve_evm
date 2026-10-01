// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { SECP256K1_ORDER } from "@ethereumjs/util";
import { createPublicClient, http, parseTransaction, serializeTransaction, toHex, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { requireRpcError, rpcRequest } from "../transport/rpc_request.js";

export async function verifyRpcRejections(context: AcceptanceContext): Promise<void> {
  const owner = context.accounts[4];
  const recipient = context.accounts[5];
  const primary = context.nodes[0];
  assert.ok(owner && recipient && primary);
  const client = createPublicClient({ chain: context.chain, transport: http(primary.httpUrl) });
  const nonce = await client.getTransactionCount({ address: owner.address });
  const fields = { type: "eip1559" as const, chainId: 31337, nonce, gas: 21_000n,
    maxFeePerGas: 2_000_000_000n, maxPriorityFeePerGas: 1_000n, to: recipient.address, value: 0n };
  const valid = await owner.signTransaction(fields);
  const parsed = parseTransaction(valid);
  assert.ok(parsed.type === "eip1559" && parsed.r && parsed.s);
  const highS = serializeTransaction(parsed, {
    r: parsed.r, s: toHex(SECP256K1_ORDER - BigInt(parsed.s), { size: 32 }), yParity: parsed.yParity === 0 ? 1 : 0,
  });
  const negative: Hex[] = [
    "0x", "0xc0", "0x02", "0x03c0", "0x04c0", "0xdeadbeef", `${valid}00`, highS,
    `0x02${"00".repeat(131_072)}`,
    await owner.signTransaction({ ...fields, chainId: 1 }),
    await owner.signTransaction({ ...fields, gas: 20_999n }),
    await owner.signTransaction({ ...fields, maxFeePerGas: 0n, maxPriorityFeePerGas: 0n }),
    await owner.signTransaction({ ...fields, value: 10n ** 40n }),
    await owner.signTransaction({ type: "legacy", nonce, gas: 21_000n, gasPrice: 2_000_000_000n,
      to: recipient.address, value: 0n }),
  ];
  for (const node of context.nodes) {
    const before = await rpcRequest(node.httpUrl, "eth_getBlockByNumber", ["latest", false]);
    const balance: Hex = await rpcRequest<Hex>(node.httpUrl, "eth_getBalance", [owner.address, "latest"]);
    for (const raw of negative) await requireRpcError(node.httpUrl, "eth_sendRawTransaction", [raw]);
    assert.deepEqual(await rpcRequest(node.httpUrl, "eth_getBlockByNumber", ["latest", false]), before);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getBalance", [owner.address, "latest"]), balance);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionCount", [owner.address, "latest"]), toHex(nonce));
  }
}

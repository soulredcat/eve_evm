// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { keccak256, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { rpcRequest } from "../transport/rpc_request.js";
import { startDevelopmentNode } from "../process/start_development_node.js";
import { stopDevelopmentNode } from "../process/stop_development_node.js";
import { submitTransaction } from "../transactions/submit_transaction.js";

export async function verifyRestart(context: AcceptanceContext): Promise<void> {
  const primary = context.nodes[0];
  const owner = context.accounts[4];
  const recipient = context.accounts[5];
  assert.ok(primary && owner && recipient);
  const head = await rpcRequest<Record<string, unknown>>(primary.httpUrl, "eth_getBlockByNumber", ["latest", false]);
  const balances = await Promise.all(context.nodes.map((node) => rpcRequest(node.httpUrl, "eth_getBalance", [owner.address, "latest"])));
  const nonces = await Promise.all(context.nodes.map((node) => rpcRequest(node.httpUrl, "eth_getTransactionCount", [owner.address, "latest"])));
  const receipts: unknown[] = [];
  for (const raw of context.submitted) {
    receipts.push(await rpcRequest(primary.httpUrl, "eth_getTransactionReceipt", [keccak256(raw)]));
  }
  const dataPaths = context.nodes.map((node) => node.dataPath);
  await Promise.all(context.nodes.map(stopDevelopmentNode));
  context.nodes = [];
  for (const dataPath of dataPaths) context.nodes.push(await startDevelopmentNode(context, dataPath));
  for (const [index, node] of context.nodes.entries()) {
    assert.deepEqual(await rpcRequest(node.httpUrl, "eth_getBlockByNumber", ["latest", false]), head);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getBalance", [owner.address, "latest"]), balances[index]);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionCount", [owner.address, "latest"]), nonces[index]);
    for (const receipt of receipts as { transactionHash: Hex }[]) {
      assert.deepEqual(await rpcRequest(node.httpUrl, "eth_getTransactionReceipt", [receipt.transactionHash]), receipt);
    }
  }
  const continued = (await submitTransaction(context, { to: recipient.address, value: 1n }))[0];
  assert.ok(continued);
  assert.equal(continued.blockNumber, BigInt(head.number as Hex) + 1n);
  assert.equal(continued.status, "success");
}

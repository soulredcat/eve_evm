// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, http, keccak256, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { rpcRequest, requireRpcError } from "../transport/rpc_request.js";
import { submitTransaction } from "../transactions/submit_transaction.js";
import { verifyRpcSurface } from "./verify_rpc_surface.js";

export async function verifyPendingReplacement(context: AcceptanceContext): Promise<void> {
  const owner = context.accounts[4];
  const recipient = context.accounts[5];
  const primary = context.nodes[0];
  assert.ok(owner && recipient && primary);
  const client = createPublicClient({ chain: context.chain, transport: http(primary.httpUrl) });
  const nonce = await client.getTransactionCount({ address: owner.address });
  const fields = { type: "eip1559" as const, chainId: 31337, nonce: nonce + 1, gas: 21_000n,
    maxFeePerGas: 2_000_000_000n, maxPriorityFeePerGas: 1_000n, to: recipient.address, value: 7n };
  const future = await owner.signTransaction(fields);
  const hash = keccak256(future);
  const underpriced = await owner.signTransaction({ ...fields, maxFeePerGas: 2_200_000_000n, maxPriorityFeePerGas: 1_099n });
  const replacement = await owner.signTransaction({ ...fields, maxFeePerGas: 2_200_000_000n, maxPriorityFeePerGas: 1_100n });
  const replacementHash = keccak256(replacement);
  for (const node of context.nodes) {
    const head = await rpcRequest(node.httpUrl, "eth_blockNumber");
    assert.equal(await rpcRequest(node.httpUrl, "eth_sendRawTransaction", [future]), hash);
    assert.equal(await rpcRequest(node.httpUrl, "eth_sendRawTransaction", [future]), hash);
    const pending = await rpcRequest<Record<string, unknown>>(node.httpUrl, "eth_getTransactionByHash", [hash]);
    assert.equal(pending.blockHash, null);
    assert.equal(pending.blockNumber, null);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionReceipt", [hash]), null);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionCount", [owner.address, "pending"]), `0x${nonce.toString(16)}`);
    await requireRpcError(node.httpUrl, "eth_sendRawTransaction", [underpriced]);
    assert.equal(await rpcRequest(node.httpUrl, "eth_sendRawTransaction", [replacement]), replacementHash);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionByHash", [hash]), null);
    assert.equal(await rpcRequest(node.httpUrl, "eth_blockNumber"), head, "A future nonce gap cannot become an included block");
  }
  context.submitted.push(replacement);
  const current = (await submitTransaction(context, { to: recipient.address, value: 3n }))[0];
  assert.ok(current);
  for (const node of context.nodes) {
    const futureReceipt = await createPublicClient({ chain: context.chain, transport: http(node.httpUrl) })
      .waitForTransactionReceipt({ hash: replacementHash, timeout: 30_000, pollingInterval: 25 });
    assert.equal(futureReceipt.status, "success");
    assert.equal(futureReceipt.blockHash, current.blockHash);
    assert.equal(futureReceipt.transactionIndex, 1);
    assert.equal(current.transactionIndex, 0);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionReceipt", [hash]), null);
    assert.equal(await rpcRequest<Hex>(node.httpUrl, "eth_getTransactionCount", [owner.address, "pending"]), `0x${(nonce + 2).toString(16)}`);
  }
  await verifyRpcSurface(context);
}

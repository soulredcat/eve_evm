// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { toHex, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { requireRpcError, rpcRequest } from "../transport/rpc_request.js";

export async function verifyHistoryAndFinality(context: AcceptanceContext): Promise<void> {
  const owner = context.accounts[4];
  const unknown = `0x${"42".repeat(32)}`;
  assert.ok(owner);
  for (const node of context.nodes) {
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionByHash", [unknown]), null);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionReceipt", [unknown]), null);
    assert.equal(await rpcRequest(node.httpUrl, "eth_getBlockByHash", [unknown, false]), null);
    const height = await rpcRequest<Hex>(node.httpUrl, "eth_blockNumber");
    assert.equal(await rpcRequest(node.httpUrl, "eth_getBlockByNumber", [toHex(BigInt(height) + 1n), false]), null);
    await requireRpcError(node.httpUrl, "eth_getBalance", [owner.address, toHex(BigInt(height) + 1n)]);
    await requireRpcError(node.httpUrl, "eth_getBalance", [owner.address, "safe"]);
    await requireRpcError(node.httpUrl, "eth_getBalance", [owner.address, "finalized"]);
    await requireRpcError(node.httpUrl, "eth_getBlockByNumber", ["safe", false]);
    await requireRpcError(node.httpUrl, "eth_getBlockByNumber", ["finalized", false]);
    await requireRpcError(node.httpUrl, "eve_getFinalityProof", [height]);
    const status = await rpcRequest<Record<string, unknown>>(node.httpUrl, "eve_getNodeStatus");
    assert.equal(status.verification_mode, "LOCAL_DEV_UNAUTHENTICATED");
    assert.equal(status.authenticated_finality, false);
    assert.equal(status.authenticated_height, null, "Local storage must not manufacture an authenticated height");
    assert.equal(BigInt(String(status.applied_height)), BigInt(height));
    assert.ok(BigInt(String(status.durable_height)) <= BigInt(String(status.applied_height)));
    await requireRpcError(node.httpUrl, "eth_getBalance", ["0xabc", "latest"], -32602);
    await requireRpcError(node.httpUrl, "eth_getCode", [owner.address, "0x00"], -32602);
    await requireRpcError(node.httpUrl, "eth_getStorageAt", [owner.address, "0x0", "0x01"], -32602);
    const genesis = await rpcRequest<Record<string, unknown>>(node.httpUrl, "eth_getBlockByNumber", ["0x0", false]);
    assert.equal(genesis.number, "0x0");
    assert.equal(await rpcRequest(node.httpUrl, "eth_getTransactionCount", [owner.address, "0x0"]), "0x0");
    assert.equal(await rpcRequest(node.httpUrl, "eth_getBalance", [owner.address, "0x0"]), toHex(100_000n * 10n ** 18n));
  }
}

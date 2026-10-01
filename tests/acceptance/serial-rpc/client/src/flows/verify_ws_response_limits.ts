// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { decodeFunctionResult, encodeFunctionData, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { deployContract } from "../contracts/deploy_contract.js";
import { rpcRequest } from "../transport/rpc_request.js";
import { wsRequest } from "../transport/ws_request.js";
import { verifyOversizedLogSubscription } from "./verify_oversized_log_subscription.js";

export async function verifyWsResponseLimits(context: AcceptanceContext): Promise<void> {
  const node = context.nodes[0];
  const owner = context.accounts[4];
  const artifact = context.contracts.get("LargeReturnProbe");
  assert.ok(node && owner && artifact);
  const probe = await deployContract(context, "LargeReturnProbe");
  const head = await rpcRequest<Hex>(node.httpUrl, "eth_blockNumber");
  const data = encodeFunctionData({ abi: artifact.abi, functionName: "returnData", args: [40_000n] });
  const params = [{ from: owner.address, to: probe, data }, head];
  const large = await wsRequest(node.wsUrl, JSON.stringify({ jsonrpc: "2.0", id: "large-result-id", method: "eth_call", params }));
  assert.equal(large.id, "large-result-id");
  assert.ok(large.error && large.error.code >= -32099 && large.error.code <= -32000,
    "A valid execution larger than the WS frame cap must return an explicit bounded transport error");
  assert.equal("result" in large, false);
  const httpResult = await rpcRequest<Hex>(node.httpUrl, "eth_call", params);
  const decoded = decodeFunctionResult({ abi: artifact.abi, functionName: "returnData", data: httpResult }) as Hex;
  assert.equal(decoded, `0x${"00".repeat(40_000)}`, "HTTP fallback must preserve the actual return bytes");
  const stringId = "x".repeat(60_000);
  const identity = await wsRequest(node.wsUrl, JSON.stringify({ jsonrpc: "2.0", id: stringId, method: "eth_chainId" }));
  assert.equal(identity.id, stringId);
  assert.equal(identity.result, "0x7a69");
  const rejected = await wsRequest(node.wsUrl, JSON.stringify({ jsonrpc: "2.0", id: "invalid-data-id", method: "eth_call", params: [{ to: probe, data: "0x1" }] }));
  assert.equal(rejected.id, "invalid-data-id");
  assert.equal(rejected.error?.code, -32602, "Odd-nibble DATA is invalid, while quantity rules remain separate");
  const malformed = await wsRequest(node.wsUrl, "{");
  assert.equal(malformed.id, null);
  assert.equal(malformed.error?.code, -32700);
  assert.equal(await rpcRequest(node.httpUrl, "eth_blockNumber"), head, "Transport errors and simulations must not publish state");
  await verifyOversizedLogSubscription(context, probe);
}

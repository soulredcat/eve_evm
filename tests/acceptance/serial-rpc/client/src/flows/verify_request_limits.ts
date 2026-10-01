// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import type { AcceptanceContext, JsonRpcEnvelope } from "../types/acceptance_types.js";
import { requireRpcError, rpcRequest } from "../transport/rpc_request.js";

export async function verifyRequestLimits(context: AcceptanceContext): Promise<void> {
  const node = context.nodes[0];
  assert.ok(node);
  const send = async (body: string): Promise<Response> => fetch(node.httpUrl, {
    method: "POST", headers: { "content-type": "application/json" }, body, signal: AbortSignal.timeout(10_000),
  });
  const omitted = await send(JSON.stringify({ jsonrpc: "2.0", id: 23, method: "eth_chainId" }));
  assert.equal((await omitted.json() as JsonRpcEnvelope).result, "0x7a69", "JSON-RPC params are optional for zero-argument methods");
  const malformed = await send("{");
  assert.equal((await malformed.json() as JsonRpcEnvelope).error?.code, -32700);
  const invalid = await send(JSON.stringify({ jsonrpc: "2.1", id: 5, method: "eth_chainId", params: [] }));
  assert.equal((await invalid.json() as JsonRpcEnvelope).error?.code, -32600);
  const batch = await send(JSON.stringify([
    { jsonrpc: "2.0", id: 7, method: "eth_chainId", params: [] },
    { jsonrpc: "2.0", method: "eth_chainId", params: [] },
    { jsonrpc: "2.0", id: "text-id", method: "eth_blockNumber", params: [] },
  ]));
  const answers = await batch.json() as JsonRpcEnvelope[];
  assert.equal(answers.length, 2, "Notifications have no response member");
  assert.deepEqual(answers.map((answer) => answer.id).sort(), [7, "text-id"].sort());
  const largeBatch = await send(JSON.stringify(Array.from({ length: 101 }, (_, id) => ({ jsonrpc: "2.0", id, method: "eth_chainId", params: [] }))));
  const limited = await largeBatch.json() as JsonRpcEnvelope;
  assert.ok(limited.error && limited.error.code >= -32099 && limited.error.code <= -32000,
    "The declared 100-request batch cap must reject 101 valid requests with an explicit server error");
  const validRequest = JSON.stringify({ jsonrpc: "2.0", id: 9, method: "eth_chainId", params: [] });
  const atLimit = await send(validRequest.padEnd(1_048_576, " "));
  assert.equal((await atLimit.json() as JsonRpcEnvelope).result, "0x7a69", "A valid request at the exact declared body cap must remain usable");
  const oversized = await send(JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_call", params: [{ data: "0x" + "00".repeat(524_288) }, "latest"] }));
  if (oversized.status === 200) {
    const error = (await oversized.json() as JsonRpcEnvelope).error;
    assert.ok(error && error.code >= -32099 && error.code <= -32000);
  } else {
    assert.ok(oversized.status === 413 || oversized.status === 400, "The 1 MiB body cap must reject the oversized call");
  }
  await requireRpcError(node.httpUrl, "eth_getLogs", [{ fromBlock: "0x0", toBlock: "0x3e8" }]);
  const latest = await rpcRequest<string>(node.httpUrl, "eth_blockNumber");
  assert.equal(typeof latest, "string", "Hostile requests must leave ordinary RPC progress available");
}

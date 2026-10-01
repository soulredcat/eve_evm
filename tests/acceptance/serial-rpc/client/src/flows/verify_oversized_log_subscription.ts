// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { decodeEventLog, type Address, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { writeContract } from "../contracts/write_contract.js";
import { rpcRequest } from "../transport/rpc_request.js";

export async function verifyOversizedLogSubscription(context: AcceptanceContext, probe: Address): Promise<void> {
  const node = context.nodes[0];
  const artifact = context.contracts.get("LargeReturnProbe");
  assert.ok(node && artifact);
  const socket = new WebSocket(node.wsUrl);
  let subscription = "";
  let rejectError: (error: unknown) => void = () => undefined;
  let rejectStarted: (error: unknown) => void = () => undefined;
  const limited = new Promise<string>((resolve, reject) => {
    rejectError = reject;
    socket.addEventListener("message", (event: MessageEvent) => {
      try {
        assert.ok(Buffer.byteLength(String(event.data)) <= 65_536 + 2_048);
        const message = JSON.parse(String(event.data)) as { method?: string; params?: { subscription: string; error?: string; result?: unknown } };
        if (message.method !== "eth_subscription") return;
        assert.equal(message.params?.subscription, subscription);
        assert.equal(message.params?.result, undefined, "Oversized logs cannot enter a success-shaped frame");
        assert.match(String(message.params?.error), /SUBSCRIPTION_FRAME_LIMIT/);
        resolve(String(message.params?.error));
      } catch (error) { reject(error); }
    });
  });
  void limited.catch(() => undefined);
  const timer = setTimeout(() => {
    const error = new Error("Oversized notification was silently dropped without an explicit limit notification");
    rejectError(error); rejectStarted(error);
  }, 10_000);
  try {
    await new Promise<void>((resolve, reject) => {
      rejectStarted = reject;
      socket.addEventListener("error", () => { const error = new Error("Subscription transport failed"); reject(error); rejectError(error); });
      socket.addEventListener("open", () => socket.send(JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_subscribe", params: ["logs", { address: probe }] })));
      socket.addEventListener("message", (event: MessageEvent) => {
        try {
          const message = JSON.parse(String(event.data)) as { id?: number; result?: string; error?: unknown };
          if (message.id !== 1) return;
          assert.equal(message.error, undefined);
          assert.equal(typeof message.result, "string");
          subscription = message.result!;
          resolve();
        } catch (error) { reject(error); }
      });
    });
    const [receipt] = await writeContract(context, "LargeReturnProbe", probe, "emitData", [40_000n]);
    assert.ok(receipt && receipt.status === "success");
    await limited;
    const historical = await rpcRequest<{ data: Hex; topics: [Hex, ...Hex[]] }[]>(node.httpUrl, "eth_getLogs", [{ address: probe, blockHash: receipt.blockHash }]);
    assert.equal(historical.length, 1);
    const decoded = decodeEventLog({ abi: artifact.abi, data: historical[0]!.data, topics: historical[0]!.topics });
    assert.equal(decoded.eventName, "Data");
    assert.deepEqual(decoded.args, { value: `0x${"00".repeat(40_000)}` }, "HTTP history preserves the full event despite the WS frame limit");
    assert.equal(await rpcRequest(node.httpUrl, "net_listening"), true);
  } finally {
    clearTimeout(timer);
    socket.close();
  }
}

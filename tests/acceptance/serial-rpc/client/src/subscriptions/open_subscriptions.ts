// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import type { Address } from "viem";

export interface SubscriptionFixture {
  socket: WebSocket;
  heads: Record<string, unknown>[];
  logs: Record<string, unknown>[];
  headId: string;
  logId: string;
  errors: Error[];
}

export async function openSubscriptions(url: string, token: Address): Promise<SubscriptionFixture> {
  const socket = new WebSocket(url);
  const heads: Record<string, unknown>[] = [];
  const logs: Record<string, unknown>[] = [];
  const errors: Error[] = [];
  let headId = "";
  let logId = "";
  try {
    await new Promise<void>((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error("Real WS subscriptions timed out")), 10_000);
      socket.addEventListener("error", () => { clearTimeout(timer); reject(new Error("WS subscription transport failed")); });
      socket.addEventListener("open", () => {
        socket.send(JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_subscribe", params: ["newHeads"] }));
        socket.send(JSON.stringify({ jsonrpc: "2.0", id: 2, method: "eth_subscribe", params: ["logs", { address: token }] }));
      });
      socket.addEventListener("message", (event: MessageEvent) => {
        try {
        const message = JSON.parse(String(event.data)) as {
          id?: number; result?: string; error?: unknown;
          method?: string; params?: { subscription: string; result: Record<string, unknown> };
        };
        if (message.id === 1 || message.id === 2) {
          if (message.error !== undefined || typeof message.result !== "string") {
            clearTimeout(timer); reject(new Error("eth_subscribe must return a real subscription ID")); return;
          }
          if (message.id === 1) headId = message.result;
          else logId = message.result;
          if (headId && logId) { clearTimeout(timer); resolve(); }
        } else if (message.method === "eth_subscription" && message.params) {
          if (message.params.subscription === headId) heads.push(message.params.result);
          else if (message.params.subscription === logId) logs.push(message.params.result);
          else errors.push(new Error("Notification does not belong to a task subscription"));
        }
        } catch (error) {
          clearTimeout(timer);
          const failure = error instanceof Error ? error : new Error(String(error));
          errors.push(failure);
          reject(failure);
        }
      });
    });
    assert.notEqual(headId, logId);
    return { socket, heads, logs, headId, logId, errors };
  } catch (error) {
    socket.close();
    throw error;
  }
}

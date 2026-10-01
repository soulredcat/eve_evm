// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import type { JsonRpcEnvelope } from "../types/acceptance_types.js";

export async function wsRequest(url: string, body: string): Promise<JsonRpcEnvelope> {
  const socket = new WebSocket(url);
  try {
    return await new Promise<JsonRpcEnvelope>((resolve, reject) => {
      const timer = setTimeout(() => reject(new Error("Real WS request timed out without an explicit result/error")), 10_000);
      socket.addEventListener("open", () => socket.send(body));
      socket.addEventListener("error", () => { clearTimeout(timer); reject(new Error("WS request transport failed")); });
      socket.addEventListener("message", (event: MessageEvent) => {
        try {
          const data = String(event.data);
          assert.ok(Buffer.byteLength(data) <= 65_536 + 2_048, "Complete WS result/error frame exceeds its declared bound");
          const envelope = JSON.parse(data) as JsonRpcEnvelope;
          assert.equal(envelope.jsonrpc, "2.0");
          assert.notEqual("result" in envelope, "error" in envelope);
          clearTimeout(timer);
          resolve(envelope);
        } catch (error) { clearTimeout(timer); reject(error); }
      });
      socket.addEventListener("close", () => { clearTimeout(timer); reject(new Error("WS closed without a response")); });
    });
  } finally {
    socket.close();
  }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import type { SubscriptionFixture } from "./open_subscriptions.js";

export async function unsubscribeFixture(fixture: SubscriptionFixture): Promise<void> {
  try {
    await new Promise<void>((resolve, reject) => {
      const replies = new Set<number>();
      const timer = setTimeout(() => reject(new Error("Real eth_unsubscribe timed out")), 10_000);
      fixture.socket.addEventListener("message", (event: MessageEvent) => {
        try {
        const message = JSON.parse(String(event.data)) as { id?: number; result?: unknown };
        if (message.id !== 3 && message.id !== 4) return;
        if (message.result !== true) {
          clearTimeout(timer); reject(new Error("Both real subscriptions must be removed")); return;
        }
        replies.add(message.id);
        if (replies.size === 2) { clearTimeout(timer); resolve(); }
        } catch (error) { clearTimeout(timer); reject(error); }
      });
      fixture.socket.send(JSON.stringify({ jsonrpc: "2.0", id: 3, method: "eth_unsubscribe", params: [fixture.headId] }));
      fixture.socket.send(JSON.stringify({ jsonrpc: "2.0", id: 4, method: "eth_unsubscribe", params: [fixture.logId] }));
    });
  } finally {
    fixture.socket.close();
  }
}

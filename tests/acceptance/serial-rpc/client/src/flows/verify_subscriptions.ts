// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { setTimeout } from "node:timers/promises";
import type { AcceptanceContext, TokenFixture } from "../types/acceptance_types.js";
import { writeContract } from "../contracts/write_contract.js";
import { rpcRequest } from "../transport/rpc_request.js";
import { wsRequest } from "../transport/ws_request.js";
import { openSubscriptions, type SubscriptionFixture } from "../subscriptions/open_subscriptions.js";
import { unsubscribeFixture } from "../subscriptions/unsubscribe_fixture.js";

export async function verifySubscriptions(context: AcceptanceContext, fixture: TokenFixture): Promise<void> {
  const recipient = context.accounts[5];
  assert.ok(recipient);
  const subscriptions: SubscriptionFixture[] = [];
  try {
    for (const node of context.nodes) subscriptions.push(await openSubscriptions(node.wsUrl, fixture.tokenA));
    for (const [index, subscription] of subscriptions.entries()) {
      const node = context.nodes[index];
      assert.ok(node);
      const isolated = await wsRequest(node.wsUrl, JSON.stringify({
        jsonrpc: "2.0", id: "another-connection", method: "eth_unsubscribe", params: [subscription.headId],
      }));
      assert.equal(isolated.error, undefined);
      assert.equal(isolated.result, false, "A separate connection cannot cancel another connection's subscription");
    }
    const receipt = (await writeContract(context, "TestERC20", fixture.tokenA, "transfer", [recipient.address, 3n]))[0];
    assert.ok(receipt);
    const deadline = Date.now() + 10_000;
    while (subscriptions.some((subscription) => subscription.heads.length === 0 || subscription.logs.length === 0)) {
      assert.ok(Date.now() < deadline, "Real head/log notifications must arrive after inclusion");
      await setTimeout(10);
    }
    for (const [index, subscription] of subscriptions.entries()) {
      const node = context.nodes[index];
      assert.ok(node);
      assert.deepEqual(subscription.errors, []);
      assert.equal(subscription.heads.length, 1);
      assert.equal(subscription.logs.length, 1);
      assert.equal(subscription.heads[0]?.hash, receipt.blockHash);
      assert.equal(subscription.logs[0]?.transactionHash, receipt.transactionHash);
      assert.equal(subscription.logs[0]?.removed, false);
      const historical: Record<string, unknown>[] = await rpcRequest<Record<string, unknown>[]>(node.httpUrl, "eth_getLogs", [{
        address: fixture.tokenA, fromBlock: `0x${receipt.blockNumber.toString(16)}`, toBlock: `0x${receipt.blockNumber.toString(16)}`,
      }]);
      assert.equal(historical.length, 1);
      assert.deepEqual(subscription.logs[0], historical[0], "Live/historical logs must preserve the same ordering and identity");
    }
  } finally {
    await Promise.all(subscriptions.map(unsubscribeFixture));
  }
}

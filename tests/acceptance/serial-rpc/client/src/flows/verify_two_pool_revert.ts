// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, http } from "viem";
import type { AcceptanceContext, TokenFixture } from "../types/acceptance_types.js";
import { writeContract } from "../contracts/write_contract.js";
import { captureSwapState } from "./capture_swap_state.js";

export async function verifyTwoPoolRevert(context: AcceptanceContext, fixture: TokenFixture): Promise<void> {
  const owner = context.accounts[4];
  const recipient = context.accounts[5];
  const node = context.nodes[0];
  assert.ok(owner && recipient && node);
  const client = createPublicClient({ chain: context.chain, transport: http(node.httpUrl) });
  const beforeState = await captureSwapState(context, fixture);
  const beforeBalance = await client.getBalance({ address: owner.address });
  const beforeNonce = await client.getTransactionCount({ address: owner.address });
  const receipt = (await writeContract(context, "AtomicSwapRouter", fixture.router, "swapTwo", [
    1_000n, 1n, 10n ** 30n, recipient.address,
  ], 500_000n))[0];
  assert.ok(receipt);
  assert.equal(receipt.status, "reverted", "A natural second-leg slippage failure must remain an included revert");
  assert.equal(receipt.logs.length, 0, "Earlier transfers/approvals/first-pool events must roll back");
  assert.deepEqual(await captureSwapState(context, fixture), beforeState, "All three tokens, both pools, allowances and router balances must roll back");
  assert.equal(await client.getBalance({ address: owner.address }), beforeBalance - receipt.gasUsed * receipt.effectiveGasPrice);
  assert.equal(await client.getTransactionCount({ address: owner.address }), beforeNonce + 1);
}

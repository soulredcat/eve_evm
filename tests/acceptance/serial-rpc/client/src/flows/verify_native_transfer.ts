// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, http } from "viem";
import { nodePool, validatorPool, type AcceptanceContext } from "../types/acceptance_types.js";
import { submitTransaction } from "../transactions/submit_transaction.js";

export async function verifyNativeTransfer(context: AcceptanceContext): Promise<void> {
  const node = context.nodes[0];
  const sender = context.accounts[4];
  const recipient = context.accounts[5];
  assert.ok(node && sender && recipient);
  const client = createPublicClient({ chain: context.chain, transport: http(node.httpUrl) });
  const addresses = [...new Set([
    ...context.accounts.map((account) => account.address),
    "0x0000000000000000000000000000000000000000" as const,
    "0x000000000000000000000000000000000000f100" as const,
    nodePool, validatorPool,
  ])];
  const beforeValues = await Promise.all(addresses.map((address) => client.getBalance({ address })));
  const before = new Map(addresses.map((address, index) => [address.toLowerCase(), beforeValues[index]!]));
  const nonce = await client.getTransactionCount({ address: sender.address });
  const receipts = await submitTransaction(context, { to: recipient.address, value: 123_456_789n });
  const receipt = receipts[0];
  assert.ok(receipt);
  assert.equal(receipt.status, "success");
  assert.equal(receipt.gasUsed, 21_000n);
  const fee = receipt.gasUsed * receipt.effectiveGasPrice;
  const burn = fee * 4_000n / 10_000n;
  const nodeCredit = fee * 3_000n / 10_000n;
  const validatorCredit = fee - burn - nodeCredit;
  const afterValues = await Promise.all(addresses.map((address) => client.getBalance({ address })));
  const after = new Map(addresses.map((address, index) => [address.toLowerCase(), afterValues[index]!]));
  assert.equal(after.get(sender.address.toLowerCase()), before.get(sender.address.toLowerCase())! - 123_456_789n - fee);
  assert.equal(after.get(recipient.address.toLowerCase()), before.get(recipient.address.toLowerCase())! + 123_456_789n);
  assert.equal(after.get(nodePool), before.get(nodePool)! + nodeCredit);
  assert.equal(after.get(validatorPool), before.get(validatorPool)! + validatorCredit);
  const changed = new Set([sender.address.toLowerCase(), recipient.address.toLowerCase(), nodePool, validatorPool]);
  for (const address of addresses) {
    if (!changed.has(address.toLowerCase())) assert.equal(after.get(address.toLowerCase()), before.get(address.toLowerCase()),
      "Every development validator, zero COINBASE and custody escrow must receive no extra fee or mint");
  }
  assert.equal(afterValues.reduce((sum, value) => sum + value, 0n), beforeValues.reduce((sum, value) => sum + value, 0n) - burn);
  assert.equal(await client.getTransactionCount({ address: sender.address }), nonce + 1);
}

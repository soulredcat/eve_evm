// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, http, keccak256, type Address, type Hex, type TransactionReceipt } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { rpcRequest } from "../transport/rpc_request.js";

export interface TransactionInput {
  to?: Address;
  data?: Hex;
  value?: bigint;
  gas?: bigint;
  accountIndex?: number;
}

export async function submitTransaction(context: AcceptanceContext, input: TransactionInput): Promise<TransactionReceipt[]> {
  const account = context.accounts[input.accountIndex ?? 4];
  const primary = context.nodes[0];
  assert.ok(account && primary);
  const client = createPublicClient({ chain: context.chain, transport: http(primary.httpUrl) });
  const nonce = await client.getTransactionCount({ address: account.address, blockTag: "pending" });
  const fields = {
    ...(input.to === undefined ? {} : { to: input.to }),
    ...(input.data === undefined ? {} : { data: input.data }),
    value: input.value ?? 0n,
  };
  const gas = input.gas ?? await client.estimateGas({ account: account.address, ...fields });
  const raw = await account.signTransaction({
    type: "eip1559", chainId: context.chain.id, nonce, gas,
    maxFeePerGas: 2_000_000_000n, maxPriorityFeePerGas: 1_000n, ...fields,
  });
  const hash = keccak256(raw);
  context.submitted.push(raw);
  const receipts = await Promise.all(context.nodes.map(async (node) => {
    assert.equal(await rpcRequest<Hex>(node.httpUrl, "eth_sendRawTransaction", [raw]), hash);
    return createPublicClient({ chain: context.chain, transport: http(node.httpUrl) })
      .waitForTransactionReceipt({ hash, timeout: 30_000, pollingInterval: 25 });
  }));
  for (const receipt of receipts) {
    assert.equal(receipt.transactionHash, hash);
    assert.equal(receipt.from.toLowerCase(), account.address.toLowerCase());
  }
  assert.equal(receipts.length, 2, "Two separate RPC processes and repositories must execute the same signed bytes");
  assert.deepEqual(receipts[0], receipts[1], "Independent serial executions must return exact receipts");
  return receipts;
}

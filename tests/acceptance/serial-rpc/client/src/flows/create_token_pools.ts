// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, decodeEventLog, http } from "viem";
import type { AcceptanceContext, TokenFixture } from "../types/acceptance_types.js";
import { deployContract } from "../contracts/deploy_contract.js";
import { writeContract } from "../contracts/write_contract.js";
import { readContract } from "../contracts/read_contract.js";

export async function createTokenPools(context: AcceptanceContext): Promise<TokenFixture> {
  const owner = context.accounts[4];
  const recipient = context.accounts[5];
  const node = context.nodes[0];
  assert.ok(owner && recipient && node);
  const tokenA = await deployContract(context, "TestERC20", ["Disposable A", "TA", 1_000_000n]);
  const tokenB = await deployContract(context, "TestERC20", ["Disposable B", "TB", 1_000_000n]);
  const tokenC = await deployContract(context, "TestERC20", ["Disposable C", "TC", 1_000_000n]);
  const artifact = context.contracts.get("TestERC20");
  assert.ok(artifact);
  const client = createPublicClient({ chain: context.chain, transport: http(node.httpUrl) });
  assert.ok((await client.getCode({ address: tokenA }))?.length! > 2);
  const transferred = (await writeContract(context, "TestERC20", tokenA, "transfer", [recipient.address, 17n]))[0];
  assert.ok(transferred);
  assert.equal(transferred.status, "success");
  assert.equal(await readContract<bigint>(context, "TestERC20", tokenA, "balanceOf", [recipient.address]), 17n);
  assert.equal(await readContract<bigint>(context, "TestERC20", tokenA, "balanceOf", [owner.address]), 999_983n);
  assert.equal(transferred.logs.length, 1);
  const event = decodeEventLog({ abi: artifact.abi, data: transferred.logs[0]!.data, topics: transferred.logs[0]!.topics });
  assert.equal(event.eventName, "Transfer");
  assert.deepEqual(event.args, { from: owner.address, to: recipient.address, value: 17n });
  const poolAB = await deployContract(context, "TestPool", [tokenA, tokenB]);
  const poolBC = await deployContract(context, "TestPool", [tokenB, tokenC]);
  for (const [token, pool] of [[tokenA, poolAB], [tokenB, poolAB], [tokenB, poolBC], [tokenC, poolBC]] as const) {
    assert.equal((await writeContract(context, "TestERC20", token, "approve", [pool, 100_000n]))[0]?.status, "success");
  }
  assert.equal((await writeContract(context, "TestPool", poolAB, "seed", [100_000n, 100_000n]))[0]?.status, "success");
  assert.equal((await writeContract(context, "TestPool", poolBC, "seed", [100_000n, 100_000n]))[0]?.status, "success");
  const router = await deployContract(context, "AtomicSwapRouter", [tokenA, tokenB, tokenC, poolAB, poolBC]);
  assert.equal((await writeContract(context, "TestERC20", tokenA, "approve", [router, 20_000n]))[0]?.status, "success");
  return { tokenA, tokenB, tokenC, poolAB, poolBC, router };
}

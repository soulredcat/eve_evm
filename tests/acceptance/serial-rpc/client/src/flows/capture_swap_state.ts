// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, http } from "viem";
import type { AcceptanceContext, TokenFixture } from "../types/acceptance_types.js";
import { readContract } from "../contracts/read_contract.js";

export async function captureSwapState(context: AcceptanceContext, fixture: TokenFixture): Promise<bigint[]> {
  const owner = context.accounts[4];
  const recipient = context.accounts[5];
  const node = context.nodes[0];
  assert.ok(owner && recipient && node);
  const height = await createPublicClient({ chain: context.chain, transport: http(node.httpUrl) }).getBlockNumber();
  const values: bigint[] = [];
  for (const token of [fixture.tokenA, fixture.tokenB, fixture.tokenC]) {
    for (const account of [owner.address, recipient.address, fixture.router, fixture.poolAB, fixture.poolBC]) {
      values.push(await readContract<bigint>(context, "TestERC20", token, "balanceOf", [account], height));
    }
  }
  for (const [token, sender, spender] of [
    [fixture.tokenA, owner.address, fixture.router], [fixture.tokenA, fixture.router, fixture.poolAB],
    [fixture.tokenB, fixture.router, fixture.poolBC],
  ] as const) {
    values.push(await readContract<bigint>(context, "TestERC20", token, "allowance", [sender, spender], height));
  }
  for (const pool of [fixture.poolAB, fixture.poolBC]) {
    for (const field of ["reserve0", "reserve1"]) values.push(await readContract<bigint>(context, "TestPool", pool, field, [], height));
  }
  return values;
}

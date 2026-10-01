// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { decodeEventLog } from "viem";
import type { AcceptanceContext, TokenFixture } from "../types/acceptance_types.js";
import { readContract } from "../contracts/read_contract.js";
import { writeContract } from "../contracts/write_contract.js";

export async function verifyTwoPoolSwap(context: AcceptanceContext, fixture: TokenFixture): Promise<void> {
  const recipient = context.accounts[5];
  const poolArtifact = context.contracts.get("TestPool");
  assert.ok(recipient && poolArtifact);
  const firstOut = 1_000n * 100_000n / 101_000n;
  const secondOut = firstOut * 100_000n / (100_000n + firstOut);
  assert.equal(firstOut, 990n);
  assert.equal(secondOut, 980n);
  const receipt = (await writeContract(context, "AtomicSwapRouter", fixture.router, "swapTwo", [
    1_000n, firstOut, secondOut, recipient.address,
  ]))[0];
  assert.ok(receipt);
  assert.equal(receipt.status, "success");
  assert.equal(await readContract<bigint>(context, "TestERC20", fixture.tokenC, "balanceOf", [recipient.address]), 980n);
  assert.equal(await readContract<bigint>(context, "TestPool", fixture.poolAB, "reserve0"), 101_000n);
  assert.equal(await readContract<bigint>(context, "TestPool", fixture.poolAB, "reserve1"), 99_010n);
  assert.equal(await readContract<bigint>(context, "TestPool", fixture.poolBC, "reserve0"), 100_990n);
  assert.equal(await readContract<bigint>(context, "TestPool", fixture.poolBC, "reserve1"), 99_020n);
  for (const token of [fixture.tokenA, fixture.tokenB, fixture.tokenC]) {
    assert.equal(await readContract<bigint>(context, "TestERC20", token, "balanceOf", [fixture.router]), 0n);
  }
  const events = receipt.logs.filter((log) => [fixture.poolAB.toLowerCase(), fixture.poolBC.toLowerCase()].includes(log.address.toLowerCase()))
    .map((log) => decodeEventLog({ abi: poolArtifact.abi, data: log.data, topics: log.topics }));
  assert.deepEqual(events.map((event) => event.eventName), ["Swap", "Swap"]);
  assert.deepEqual(events.map((event) => event.args), [
    { caller: fixture.router, tokenIn: fixture.tokenA, amountIn: 1_000n, amountOut: firstOut },
    { caller: fixture.router, tokenIn: fixture.tokenB, amountIn: firstOut, amountOut: secondOut },
  ]);
  assert.deepEqual(receipt.logs.map((log) => log.logIndex), receipt.logs.map((_, index) => index));
}

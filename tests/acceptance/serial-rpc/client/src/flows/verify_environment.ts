// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, http } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { deployContract } from "../contracts/deploy_contract.js";
import { readContract } from "../contracts/read_contract.js";

export async function verifyEnvironment(context: AcceptanceContext): Promise<void> {
  const node = context.nodes[0];
  assert.ok(node);
  const probe = await deployContract(context, "EnvironmentProbe");
  const client = createPublicClient({ chain: context.chain, transport: http(node.httpUrl) });
  const block = await client.getBlock();
  assert.ok(block.number !== null && block.number > 0n && block.baseFeePerGas !== null);
  const fields = await readContract<readonly [bigint, bigint, bigint, bigint, string, bigint, string, bigint]>(
    context, "EnvironmentProbe", probe, "environment", [block.number - 1n], block.number,
  );
  assert.equal(fields[0], block.number);
  assert.equal(fields[1], 31337n);
  assert.equal(fields[2], block.timestamp);
  assert.equal(fields[3], block.baseFeePerGas);
  assert.equal(fields[4].toLowerCase(), block.miner.toLowerCase());
  assert.equal(fields[5], BigInt(block.mixHash));
  assert.equal(fields[6], block.parentHash, "BLOCKHASH must bind the EVE execution header history");
  assert.equal(fields[7], block.gasLimit);
  const current = await readContract<readonly unknown[]>(context, "EnvironmentProbe", probe, "environment", [block.number], block.number);
  assert.equal(current[6], `0x${"00".repeat(32)}`, "Current execution height is outside BLOCKHASH's past-only domain");
}

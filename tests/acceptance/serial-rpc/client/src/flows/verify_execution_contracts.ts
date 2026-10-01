// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, encodeFunctionData, getCreate2Address, http, toHex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { deployContract } from "../contracts/deploy_contract.js";
import { readContract } from "../contracts/read_contract.js";
import { writeContract } from "../contracts/write_contract.js";
import { rpcRequest } from "../transport/rpc_request.js";

export async function verifyExecutionContracts(context: AcceptanceContext): Promise<void> {
  const node = context.nodes[0];
  const recipient = context.accounts[5];
  const targetArtifact = context.contracts.get("DelegateTarget");
  assert.ok(node && recipient && targetArtifact);
  const client = createPublicClient({ chain: context.chain, transport: http(node.httpUrl) });
  const factory = await deployContract(context, "Create2Factory");
  const salt = toHex(42n, { size: 32 });
  const created = getCreate2Address({ from: factory, salt, bytecode: targetArtifact.bytecode });
  assert.equal((await writeContract(context, "Create2Factory", factory, "deploy", [targetArtifact.bytecode, salt]))[0]?.status, "success");
  assert.equal(await client.getCode({ address: created }), targetArtifact.deployedBytecode);
  assert.equal((await writeContract(context, "Create2Factory", factory, "deploy", [targetArtifact.bytecode, salt], 300_000n))[0]?.status, "reverted");
  assert.equal(await readContract<bigint>(context, "DelegateTarget", created, "value"), 0n);
  const proxy = await deployContract(context, "DelegateProxy");
  const data = encodeFunctionData({ abi: targetArtifact.abi, functionName: "setValue", args: [4_242n] });
  assert.equal((await writeContract(context, "DelegateProxy", proxy, "execute", [created, data]))[0]?.status, "success");
  assert.equal(await readContract<bigint>(context, "DelegateProxy", proxy, "value"), 4_242n);
  assert.equal(await readContract<bigint>(context, "DelegateTarget", created, "value"), 0n);
  const doomed = await deployContract(context, "ShanghaiDestruction", [], 789n);
  assert.equal(await client.getStorageAt({ address: doomed, slot: toHex(0n, { size: 32 }) }), toHex(77n, { size: 32 }));
  const balance = await client.getBalance({ address: recipient.address });
  assert.equal((await writeContract(context, "ShanghaiDestruction", doomed, "destroy", [recipient.address]))[0]?.status, "success");
  assert.equal(await rpcRequest(node.httpUrl, "eth_getCode", [doomed, "latest"]), "0x", "Shanghai must delete prior-transaction contract code");
  assert.equal(await client.getCode({ address: doomed }), undefined, "Viem represents an empty code response as absent bytecode");
  assert.equal(await client.getStorageAt({ address: doomed, slot: toHex(0n, { size: 32 }) }), toHex(0n, { size: 32 }));
  assert.equal(await client.getBalance({ address: doomed }), 0n);
  assert.equal(await client.getBalance({ address: recipient.address }), balance + 789n);
  const reentrant = await deployContract(context, "TestReentrantTarget");
  const driver = await deployContract(context, "TestReentryDriver", [reentrant]);
  assert.equal((await writeContract(context, "TestReentryDriver", driver, "start", [3n, false]))[0]?.status, "success");
  assert.equal(await readContract<bigint>(context, "TestReentrantTarget", reentrant, "calls"), 4n);
  assert.equal((await writeContract(context, "TestReentryDriver", driver, "start", [3n, true], 300_000n))[0]?.status, "reverted");
  assert.equal(await readContract<bigint>(context, "TestReentrantTarget", reentrant, "calls"), 4n);
  const refund = await deployContract(context, "StorageRefundProbe");
  assert.equal((await writeContract(context, "StorageRefundProbe", refund, "populate"))[0]?.status, "success");
  for (let index = 0; index < 16; index++) assert.equal(await readContract<bigint>(context, "StorageRefundProbe", refund, "values", [BigInt(index)]), BigInt(index + 1));
  const cleared = (await writeContract(context, "StorageRefundProbe", refund, "clear", [], 200_000n))[0];
  assert.ok(cleared);
  assert.equal(cleared.status, "success");
  assert.ok(cleared.gasUsed < 200_000n);
  for (let index = 0; index < 16; index++) assert.equal(await readContract<bigint>(context, "StorageRefundProbe", refund, "values", [BigInt(index)]), 0n);
  const exhausted = (await writeContract(context, "StorageRefundProbe", refund, "exhaustGas", [], 100_000n))[0];
  assert.ok(exhausted);
  assert.equal(exhausted.status, "reverted");
  assert.equal(exhausted.gasUsed, 100_000n);
  for (let index = 0; index < 16; index++) assert.equal(await readContract<bigint>(context, "StorageRefundProbe", refund, "values", [BigInt(index)]), 0n);
}

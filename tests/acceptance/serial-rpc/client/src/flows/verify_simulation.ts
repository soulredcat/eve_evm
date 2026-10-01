// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, encodeErrorResult, encodeFunctionData, http, toHex, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { deployContract } from "../contracts/deploy_contract.js";
import { rpcEnvelope, rpcRequest } from "../transport/rpc_request.js";

export async function verifySimulation(context: AcceptanceContext): Promise<void> {
  const node = context.nodes[0];
  const owner = context.accounts[4];
  const artifact = context.contracts.get("StorageRefundProbe");
  assert.ok(node && owner && artifact);
  const probe = await deployContract(context, "StorageRefundProbe");
  const client = createPublicClient({ chain: context.chain, transport: http(node.httpUrl) });
  const block = await client.getBlock();
  const beforeNonce = await client.getTransactionCount({ address: owner.address });
  const beforeBalance = await client.getBalance({ address: owner.address });
  const data = encodeFunctionData({ abi: artifact.abi, functionName: "populate" });
  const call = { from: owner.address, to: probe, data, gas: "0x989680" };
  assert.equal(await rpcRequest<Hex>(node.httpUrl, "eth_call", [call, toHex(block.number!)]), "0x");
  const estimate = await client.estimateGas({ account: owner.address, to: probe, data, blockNumber: block.number! });
  assert.ok(estimate > 21_000n && estimate <= 10_000_000n);
  assert.equal(await rpcRequest(node.httpUrl, "eth_call", [{ ...call, gas: toHex(estimate) }, toHex(block.number!)]), "0x");
  const tooSmall = await rpcEnvelope(node.httpUrl, "eth_call", [{ ...call, gas: "0x5208" }, toHex(block.number!)]);
  assert.ok(tooSmall.error, "A contract simulation must fail with only native-transfer gas");
  const failing = encodeFunctionData({ abi: artifact.abi, functionName: "failAfterWrite", args: [55n] });
  const failed = await rpcEnvelope(node.httpUrl, "eth_call", [{ ...call, data: failing }, toHex(block.number!)]);
  assert.ok(failed.error);
  assert.equal(failed.error.code, 3, "Ethereum execution revert must carry a real execution error");
  assert.equal(failed.error.data, encodeErrorResult({ abi: artifact.abi, errorName: "DeliberateFailure", args: [55n] }));
  const failedEstimate = await rpcEnvelope(node.httpUrl, "eth_estimateGas", [{ ...call, data: failing }, toHex(block.number!)]);
  assert.ok(failedEstimate.error);
  assert.equal(failedEstimate.error.data, failed.error.data);
  for (const method of ["eth_call", "eth_estimateGas"]) {
    const wrongChain = await rpcEnvelope(node.httpUrl, method, [{ ...call, chainId: "0x1" }, toHex(block.number!)]);
    assert.equal(wrongChain.error?.code, -32602, "Unsupported chain overrides must be rejected instead of ignored");
    const creationNonce = await rpcEnvelope(node.httpUrl, method, [{
      from: owner.address, data: artifact.bytecode, gas: "0x989680", nonce: "0x2a",
    }, toHex(block.number!)]);
    assert.equal(creationNonce.error?.code, -32602, "Unsupported creation nonce overrides cannot silently change the derived contract address");
  }
  assert.equal(await client.getStorageAt({ address: probe, slot: toHex(0n, { size: 32 }) }), toHex(0n, { size: 32 }));
  assert.equal(await client.getTransactionCount({ address: owner.address }), beforeNonce);
  assert.equal(await client.getBalance({ address: owner.address }), beforeBalance);
  const after = await client.getBlock();
  assert.equal(after.hash, block.hash, "Read-only simulation/estimation must not create a block or change canonical roots");
  assert.equal(after.stateRoot, block.stateRoot);
  const native = await rpcEnvelope(node.httpUrl, "eth_call", [{ from: owner.address, to: "0x000000000000000000000000000000000000f100", data: "0x", gas: "0x186a0" }, "latest"]);
  assert.ok(native.error, "The unactivated native interface must fail closed");
  assert.equal(native.error.data, toHex("EVE_NATIVE_INTERFACE_INACTIVE_V1"));
}

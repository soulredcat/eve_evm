// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import path from "node:path";
import { createPublicClient, http, keccak256, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { startMasterComposition } from "../process/start_master_composition.js";
import { stopDevelopmentNode } from "../process/stop_development_node.js";
import { rpcRequest, requireRpcError } from "../transport/rpc_request.js";
import { verifyHeaderHash } from "../encoding/verify_header_hash.js";

export async function verifyMasterComposition(context: AcceptanceContext): Promise<void> {
  const owner = context.accounts[4];
  const recipient = context.accounts[5];
  assert.ok(owner && recipient);
  for (const [index, mode, acknowledged] of [
    [0, "PRODUCTION", true], [1, "MASTER_SYNC_ONLY", true], [2, "DEV_ALL_IN_ONE", false],
  ] as const) {
    const refused = spawnSync(context.masterBinary, [
      "serve-dev", "--root", context.repositoryRoot,
      "--data", path.relative(context.repositoryRoot, path.join(context.runDirectory, `master-refusal-${index}`)),
      "--genesis", context.genesisPath, "--mode", mode,
      ...(acknowledged ? ["--acknowledge-unsafe-development"] : []),
      "--http-address", "127.0.0.1:0", "--ws-address", "127.0.0.1:0", "--block-interval-ms", "100",
    ], { encoding: "utf8", timeout: 10_000, maxBuffer: 65_536 });
    assert.equal(refused.error, undefined, "A mode refusal must finish rather than time out after starting a listener");
    assert.notEqual(refused.status, 0, "Master composition must reject production, sync-only producer and missing acknowledgement");
    assert.match(refused.stderr, /DEV_ALL_IN_ONE/);
    assert.equal(refused.stdout.includes('"authenticated_finality"'), false);
  }
  const data = path.relative(context.repositoryRoot, path.join(context.runDirectory, "master-composition"));
  let master = await startMasterComposition(context, data);
  const index = context.extraNodes.push(master) - 1;
  try {
    const client = createPublicClient({ chain: context.chain, transport: http(master.httpUrl) });
    assert.equal(await client.getBlockNumber(), 0n);
    const before = await client.getBalance({ address: owner.address });
    const raw = await owner.signTransaction({
      type: "eip1559", chainId: 31337, nonce: 0, gas: 21_000n,
      maxFeePerGas: 2_000_000_000n, maxPriorityFeePerGas: 1000n, to: recipient.address, value: 11n,
    });
    const hash = keccak256(raw);
    assert.equal(await rpcRequest<Hex>(master.httpUrl, "eth_sendRawTransaction", [raw]), hash);
    const receipt = await client.waitForTransactionReceipt({ hash, timeout: 30_000, pollingInterval: 25 });
    assert.equal(receipt.status, "success");
    assert.equal(receipt.blockNumber, 1n);
    assert.equal(receipt.gasUsed, 21_000n);
    assert.equal(await client.getBalance({ address: owner.address }), before - 11n - receipt.gasUsed * receipt.effectiveGasPrice);
    const head = await rpcRequest<Record<string, unknown>>(master.httpUrl, "eth_getBlockByNumber", ["latest", false]);
    verifyHeaderHash(head);
    const storedReceipt = await rpcRequest(master.httpUrl, "eth_getTransactionReceipt", [hash]);
    const storedBalance = await rpcRequest(master.httpUrl, "eth_getBalance", [owner.address, "latest"]);
    await requireRpcError(master.httpUrl, "eve_getFinalityProof", ["0x1"]);
    await stopDevelopmentNode(master);
    master = await startMasterComposition(context, data);
    context.extraNodes[index] = master;
    assert.deepEqual(await rpcRequest(master.httpUrl, "eth_getBlockByNumber", ["latest", false]), head);
    assert.deepEqual(await rpcRequest(master.httpUrl, "eth_getTransactionReceipt", [hash]), storedReceipt);
    assert.equal(await rpcRequest(master.httpUrl, "eth_getBalance", [owner.address, "latest"]), storedBalance);
    assert.equal(await rpcRequest(master.httpUrl, "eth_getTransactionCount", [owner.address, "latest"]), "0x1");
    assert.equal(await rpcRequest(master.httpUrl, "eth_getBalance", [owner.address, "0x0"]), `0x${before.toString(16)}`);
    const status = await rpcRequest<Record<string, unknown>>(master.httpUrl, "eve_getNodeStatus");
    assert.equal(status.verification_mode, "LOCAL_DEV_UNAUTHENTICATED");
    assert.equal(status.authenticated_finality, false);
    assert.equal(status.authenticated_height, null);
  } finally {
    await stopDevelopmentNode(master);
  }
}

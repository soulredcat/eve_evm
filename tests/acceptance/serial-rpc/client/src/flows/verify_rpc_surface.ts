// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createMPT } from "@ethereumjs/mpt";
import { RLP } from "@ethereumjs/rlp";
import { bytesToHex, hexToBytes, keccak256, parseTransaction, type Hex } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { rpcRequest, requireRpcError } from "../transport/rpc_request.js";
import { verifyHeaderHash } from "../encoding/verify_header_hash.js";
import { encodeRpcReceipt } from "../encoding/encode_rpc_receipt.js";
import { encodeQuantity } from "../encoding/encode_quantity.js";

export async function verifyRpcSurface(context: AcceptanceContext): Promise<void> {
  const last = context.submitted.at(-1);
  assert.ok(last);
  const known = new Map(context.submitted.map((raw) => [keccak256(raw), raw]));
  let previous: Record<string, unknown> | undefined;
  for (const node of context.nodes) {
    assert.match(await rpcRequest<string>(node.httpUrl, "web3_clientVersion"), /eve/i);
    assert.equal(await rpcRequest<string>(node.httpUrl, "net_version"), "31337");
    assert.equal(await rpcRequest<boolean>(node.httpUrl, "net_listening"), true);
    assert.equal(await rpcRequest<Hex>(node.httpUrl, "eth_chainId"), "0x7a69");
    assert.equal(await rpcRequest<boolean>(node.httpUrl, "eth_syncing"), false);
    const height = await rpcRequest<Hex>(node.httpUrl, "eth_blockNumber");
    encodeQuantity(height);
    const block = await rpcRequest<Record<string, unknown>>(node.httpUrl, "eth_getBlockByNumber", [height, false]);
    verifyHeaderHash(block);
    assert.deepEqual(await rpcRequest(node.httpUrl, "eth_getBlockByHash", [block.hash, false]), block);
    const hashes = block.transactions as Hex[];
    assert.ok(Array.isArray(hashes) && hashes.length > 0);
    const transactions = await createMPT({ useKeyHashing: false });
    const receipts = await createMPT({ useKeyHashing: false });
    let cumulative = 0n;
    for (const [index, hash] of hashes.entries()) {
      const raw = known.get(hash);
      assert.ok(raw, "A task-owned chain must contain exactly known signed fixture transactions");
      const parsed = parseTransaction(raw);
      const transaction = await rpcRequest<Record<string, unknown>>(node.httpUrl, "eth_getTransactionByHash", [hash]);
      assert.equal(transaction.hash, keccak256(raw));
      assert.equal(transaction.blockHash, block.hash);
      assert.equal(BigInt(transaction.nonce as Hex), BigInt(parsed.nonce!));
      assert.equal(BigInt(transaction.value as Hex), parsed.value ?? 0n);
      assert.equal(transaction.input, parsed.data ?? "0x");
      const receipt = await rpcRequest<Record<string, unknown>>(node.httpUrl, "eth_getTransactionReceipt", [hash]);
      assert.equal(receipt.transactionHash, hash);
      assert.equal(receipt.blockHash, block.hash);
      assert.equal(BigInt(receipt.transactionIndex as Hex), BigInt(index));
      cumulative += BigInt(receipt.gasUsed as Hex);
      assert.equal(BigInt(receipt.cumulativeGasUsed as Hex), cumulative);
      await transactions.put(RLP.encode(index), hexToBytes(raw));
      await receipts.put(RLP.encode(index), encodeRpcReceipt(receipt));
    }
    assert.equal(bytesToHex(transactions.root()), block.transactionsRoot);
    assert.equal(bytesToHex(receipts.root()), block.receiptsRoot);
    assert.equal(cumulative, BigInt(block.gasUsed as Hex));
    assert.deepEqual(block.uncles, []);
    assert.deepEqual(block.withdrawals, []);
    const history = await rpcRequest<{ oldestBlock: Hex; baseFeePerGas: Hex[]; gasUsedRatio: number[]; reward?: Hex[][] }>(node.httpUrl, "eth_feeHistory", ["0x1", height, [0, 50, 100]]);
    assert.equal(history.oldestBlock, height);
    assert.equal(history.baseFeePerGas.length, 2);
    assert.equal(history.baseFeePerGas[0], block.baseFeePerGas);
    assert.equal(history.gasUsedRatio.length, 1);
    assert.equal(history.gasUsedRatio[0], Number(BigInt(block.gasUsed as Hex)) / 30_000_000);
    encodeQuantity(await rpcRequest(node.httpUrl, "eth_gasPrice"));
    encodeQuantity(await rpcRequest(node.httpUrl, "eth_maxPriorityFeePerGas"));
    await requireRpcError(node.httpUrl, "eth_methodDoesNotExist", [], -32601);
    if (previous !== undefined) assert.deepEqual(block, previous, "Independent stores must have exact header/body identity");
    previous = block;
  }
}

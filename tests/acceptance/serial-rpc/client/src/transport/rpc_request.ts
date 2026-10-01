// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import type { JsonRpcEnvelope } from "../types/acceptance_types.js";

let nextId = 1;

export async function rpcEnvelope(url: string, method: string, params: unknown[] = []): Promise<JsonRpcEnvelope> {
  const id = nextId++;
  const response = await fetch(url, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", id, method, params }),
    signal: AbortSignal.timeout(10_000),
  });
  assert.equal(response.status, 200, `Unexpected HTTP status for ${method}`);
  const envelope = (await response.json()) as JsonRpcEnvelope;
  assert.equal(envelope.jsonrpc, "2.0");
  assert.equal(envelope.id, id, "JSON-RPC response must retain the request ID");
  assert.notEqual("result" in envelope, "error" in envelope, "Exactly one result/error is required");
  return envelope;
}

export async function rpcRequest<T>(url: string, method: string, params: unknown[] = []): Promise<T> {
  const envelope = await rpcEnvelope(url, method, params);
  assert.equal(envelope.error, undefined, `${method}: ${JSON.stringify(envelope.error)}`);
  return envelope.result as T;
}

export async function requireRpcError(url: string, method: string, params: unknown[], code?: number): Promise<void> {
  const envelope = await rpcEnvelope(url, method, params);
  assert.ok(envelope.error, `${method} unexpectedly succeeded`);
  assert.equal("result" in envelope, false);
  if (code !== undefined) assert.equal(envelope.error.code, code);
}

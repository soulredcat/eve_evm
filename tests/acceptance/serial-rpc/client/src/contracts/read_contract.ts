// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { createPublicClient, http, type Address } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";

export async function readContract<T>(context: AcceptanceContext, name: string, address: Address, functionName: string, args: readonly unknown[] = [], blockNumber?: bigint): Promise<T> {
  const artifact = context.contracts.get(name);
  const node = context.nodes[0];
  assert.ok(artifact && node);
  return await createPublicClient({ chain: context.chain, transport: http(node.httpUrl) })
    .readContract({ abi: artifact.abi, address, functionName, args,
      ...(blockNumber === undefined ? {} : { blockNumber }),
    }) as T;
}

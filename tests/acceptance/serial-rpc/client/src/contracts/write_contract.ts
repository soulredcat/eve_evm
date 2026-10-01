// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { encodeFunctionData, type Address, type TransactionReceipt } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { submitTransaction } from "../transactions/submit_transaction.js";

export async function writeContract(
  context: AcceptanceContext, name: string, address: Address, functionName: string,
  args: readonly unknown[] = [], gas?: bigint,
): Promise<TransactionReceipt[]> {
  const artifact = context.contracts.get(name);
  assert.ok(artifact);
  return submitTransaction(context, {
    to: address, data: encodeFunctionData({ abi: artifact.abi, functionName, args }),
    ...(gas === undefined ? {} : { gas }),
  });
}

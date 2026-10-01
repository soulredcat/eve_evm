// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { encodeDeployData, getAddress, type Address } from "viem";
import type { AcceptanceContext } from "../types/acceptance_types.js";
import { submitTransaction } from "../transactions/submit_transaction.js";

export async function deployContract(context: AcceptanceContext, name: string, args: readonly unknown[] = [], value = 0n): Promise<Address> {
  const artifact = context.contracts.get(name);
  assert.ok(artifact, `Every deployment must use the real compiled ${name} artifact`);
  const receipts = await submitTransaction(context, {
    data: encodeDeployData({ abi: artifact.abi, bytecode: artifact.bytecode, args }), value,
  });
  const receipt = receipts[0];
  assert.ok(receipt);
  assert.equal(receipt.status, "success");
  assert.ok(receipt.contractAddress, `${name} must have an actual CREATE address`);
  return getAddress(receipt.contractAddress);
}

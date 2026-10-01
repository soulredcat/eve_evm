// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import type { Abi, Hex } from "viem";
import type { CompiledContract } from "../types/acceptance_types.js";
import { collectSoliditySources } from "./collect_solidity_sources.js";

interface CompilerOutput {
  errors?: { severity: string; formattedMessage: string }[];
  contracts?: Record<string, Record<string, {
    abi: Abi;
    evm: { bytecode: { object: string }; deployedBytecode: { object: string } };
  }>>;
}

export async function compileContracts(solc: string, sourceDirectory: string): Promise<Map<string, CompiledContract>> {
  const sources = await collectSoliditySources(sourceDirectory);
  assert.equal(Object.keys(sources).length, 12, "Every owned Solidity fixture must be compiled");
  const input = {
    language: "Solidity",
    sources,
    settings: {
      evmVersion: "shanghai",
      optimizer: { enabled: true, runs: 200 },
      metadata: { bytecodeHash: "none", appendCBOR: false },
      outputSelection: { "*": { "*": ["abi", "evm.bytecode.object", "evm.deployedBytecode.object"] } },
    },
  };
  const child = spawnSync(solc, ["--standard-json"], {
    input: JSON.stringify(input), encoding: "utf8", maxBuffer: 64 * 1024 * 1024, timeout: 60_000,
  });
  assert.equal(child.error, undefined);
  assert.equal(child.status, 0, child.stderr);
  const output = JSON.parse(child.stdout) as CompilerOutput;
  const errors = output.errors?.filter((error) => error.severity === "error") ?? [];
  assert.deepEqual(errors, []);
  const result = new Map<string, CompiledContract>();
  for (const contracts of Object.values(output.contracts ?? {})) {
    for (const [name, contract] of Object.entries(contracts)) {
      if (contract.evm.bytecode.object.length === 0) continue;
      assert.equal(result.has(name), false, `Ambiguous contract name ${name}`);
      result.set(name, {
        abi: contract.abi,
        bytecode: `0x${contract.evm.bytecode.object}` as Hex,
        deployedBytecode: `0x${contract.evm.deployedBytecode.object}` as Hex,
      });
    }
  }
  assert.equal(result.size, 12);
  return result;
}

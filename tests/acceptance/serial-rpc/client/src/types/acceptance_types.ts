// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import type { ChildProcess } from "node:child_process";
import type { Abi, Address, Chain, Hex } from "viem";
import type { PrivateKeyAccount } from "viem/accounts";

export interface CompiledContract {
  abi: Abi;
  bytecode: Hex;
  deployedBytecode: Hex;
}

export interface DevelopmentNode {
  process: ChildProcess;
  httpUrl: string;
  wsUrl: string;
  dataPath: string;
  startup: Record<string, unknown>;
  stderr: () => string;
  diagnosticPath: string;
}

export interface AcceptanceContext {
  repositoryRoot: string;
  runDirectory: string;
  publicBinary: string;
  masterBinary: string;
  genesisPath: string;
  chain: Chain;
  accounts: PrivateKeyAccount[];
  contracts: Map<string, CompiledContract>;
  nodes: DevelopmentNode[];
  extraNodes: DevelopmentNode[];
  submitted: Hex[];
}

export interface TokenFixture {
  tokenA: Address;
  tokenB: Address;
  tokenC: Address;
  poolAB: Address;
  poolBC: Address;
  router: Address;
}

export interface JsonRpcEnvelope {
  jsonrpc: string;
  id: string | number | null;
  result?: unknown;
  error?: { code: number; message: string; data?: unknown };
}

export const developmentChain: Chain = {
  id: 31337,
  name: "EVE local disposable acceptance",
  nativeCurrency: { name: "Disposable EVE", symbol: "TEST", decimals: 18 },
  rpcUrls: { default: { http: ["http://127.0.0.1:8545"] } },
};

export const nodePool = "0x000000000000000000000000000000000000f101" as const;
export const validatorPool = "0x000000000000000000000000000000000000f102" as const;

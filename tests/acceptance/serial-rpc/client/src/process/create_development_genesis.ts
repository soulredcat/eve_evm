// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import { generateKeyPairSync } from "node:crypto";
import { writeFile } from "node:fs/promises";
import { generatePrivateKey, privateKeyToAccount, type PrivateKeyAccount } from "viem/accounts";
import { toHex } from "viem";

export async function createDevelopmentGenesis(genesisPath: string): Promise<PrivateKeyAccount[]> {
  // Keys exist only in this task process. The genesis file contains public data.
  const accounts = Array.from({ length: 6 }, () => privateKeyToAccount(generatePrivateKey()));
  const unit = 10n ** 18n;
  const validators = accounts.slice(0, 4).map((account) => {
    const pair = generateKeyPairSync("ed25519");
    const publicDer = pair.publicKey.export({ type: "spki", format: "der" });
    return {
      owner: account.address,
      classical_public_key: toHex(publicDer.subarray(publicDer.length - 32)),
      self_bond: toHex(10_000n * unit),
      voting_power: 10_000,
    };
  });
  const genesis = {
    schema_version: 1, protocol_version: 1, network_name: "eve-local-v1", evm_chain_id: 31337,
    initial_timestamp: 1_728_000_000, profile: "CLASSICAL_DEV",
    accounts: accounts.map((account) => ({
      address: account.address, funded_balance: toHex(100_000n * unit), nonce: 0, code: "0x",
    })),
    validators,
  };
  await writeFile(genesisPath, JSON.stringify(genesis, null, 2) + "\n", { flag: "wx" });
  return accounts;
}

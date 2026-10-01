// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { mkdir, realpath } from "node:fs/promises";
import path from "node:path";
import { compileContracts } from "./contracts/compile_contracts.js";
import { createDevelopmentGenesis } from "./process/create_development_genesis.js";
import { startDevelopmentNode } from "./process/start_development_node.js";
import { stopDevelopmentNode } from "./process/stop_development_node.js";
import { developmentChain, type AcceptanceContext } from "./types/acceptance_types.js";
import { verifyNativeTransfer } from "./flows/verify_native_transfer.js";
import { createTokenPools } from "./flows/create_token_pools.js";
import { verifyTwoPoolSwap } from "./flows/verify_two_pool_swap.js";
import { verifyTwoPoolRevert } from "./flows/verify_two_pool_revert.js";
import { verifyExecutionContracts } from "./flows/verify_execution_contracts.js";
import { verifyEnvironment } from "./flows/verify_environment.js";
import { verifyWsResponseLimits } from "./flows/verify_ws_response_limits.js";
import { verifySimulation } from "./flows/verify_simulation.js";
import { verifyProofs } from "./flows/verify_proofs.js";
import { verifyRpcRejections } from "./flows/verify_rpc_rejections.js";
import { verifyPendingReplacement } from "./flows/verify_pending_replacement.js";
import { verifySubscriptions } from "./flows/verify_subscriptions.js";
import { verifyRpcSurface } from "./flows/verify_rpc_surface.js";
import { verifyHistoryAndFinality } from "./flows/verify_history_and_finality.js";
import { verifyRequestLimits } from "./flows/verify_request_limits.js";
import { verifyRestart } from "./flows/verify_restart.js";
import { verifyMasterComposition } from "./flows/verify_master_composition.js";

const [rootArgument, runArgument, publicBinary, solcBinary, masterBinary] = process.argv.slice(2);
assert.ok(rootArgument && runArgument && publicBinary && solcBinary && masterBinary, "Explicit verified root/run/public/solc/master paths are mandatory");
const repositoryRoot = await realpath(rootArgument);
const runDirectory = path.resolve(runArgument);
const localRoot = await realpath(path.join(repositoryRoot, "local-tests"));
const relativeRun = path.relative(localRoot, runDirectory);
assert.ok(relativeRun !== "" && !relativeRun.startsWith("..") && !path.isAbsolute(relativeRun));
await mkdir(runDirectory, { recursive: true });
assert.equal(await realpath(runDirectory), runDirectory, "Acceptance output must not escape through a symlink");
const genesisPath = path.join(runDirectory, "public-development-genesis.json");
const accounts = await createDevelopmentGenesis(genesisPath);
const contracts = await compileContracts(solcBinary, path.join(repositoryRoot, "tests/acceptance/serial-rpc/contracts"));
const context: AcceptanceContext = {
  repositoryRoot, runDirectory, publicBinary, masterBinary, genesisPath,
  chain: developmentChain, accounts, contracts, nodes: [], extraNodes: [], submitted: [],
};
const passed: string[] = [];
const emergencyStop = (code: number): void => {
  void Promise.allSettled([...context.nodes, ...context.extraNodes].map(stopDevelopmentNode)).finally(() => process.exit(code));
};
process.once("SIGTERM", () => emergencyStop(143));
process.once("SIGINT", () => emergencyStop(130));
const deadline = setTimeout(() => {
  process.stderr.write("Acceptance deadline exceeded; stopping only task-owned children\n");
  emergencyStop(124);
}, 180_000);
const run = async (name: string, operation: () => Promise<void>): Promise<void> => {
  await operation();
  passed.push(name);
  process.stdout.write(JSON.stringify({ acceptance_case: name, result: "PASS" }) + "\n");
};
try {
  for (const name of ["public-one", "public-two"]) {
    const dataPath = path.relative(repositoryRoot, path.join(runDirectory, name));
    context.nodes.push(await startDevelopmentNode(context, dataPath));
  }
  await run("ta02_native_transfer_exact_fee_value_nonce_supply", () => verifyNativeTransfer(context));
  const fixture = await createTokenPools(context);
  passed.push("ta02_real_solidity_erc20_deployment_abi_transfer_and_seed");
  await run("ta02_two_pool_success_literal_reserves_and_ordered_events", () => verifyTwoPoolSwap(context, fixture));
  await run("te02_second_leg_revert_rolls_back_all_token_pool_allowance_effects", () => verifyTwoPoolRevert(context, fixture));
  await run("te02_create2_delegatecall_shanghai_destruction_reentry_refund_out_of_gas", () => verifyExecutionContracts(context));
  await run("te03_selected_header_environment_and_blockhash_are_exact", () => verifyEnvironment(context));
  await run("ta05_ws_oversize_response_string_ids_data_errors_and_http_fallback", () => verifyWsResponseLimits(context));
  await run("te06_call_estimation_revert_data_and_inactive_native_preserve_canonical_state", () => verifySimulation(context));
  await run("ta06_independent_ethereumjs_account_storage_absence_and_corruption_proofs", () => verifyProofs(context, fixture));
  await run("te01_rpc_signed_envelope_rejections_preserve_canonical_parent", () => verifyRpcRejections(context));
  await run("ta04_rpc_pending_gap_duplicate_replacement_and_multi_receipt_order", () => verifyPendingReplacement(context));
  await run("ta06_real_websocket_heads_logs_history_and_unsubscribe", () => verifySubscriptions(context, fixture));
  await run("ta01_rpc_encoding_canonical_header_transaction_and_receipt_roots", () => verifyRpcSurface(context));
  await run("ta03_unknown_future_history_and_unauthenticated_finality_are_distinct", () => verifyHistoryAndFinality(context));
  await run("ta05_exact_request_body_batch_and_historical_query_limits", () => verifyRequestLimits(context));
  await run("ta06_actual_two_process_restart_reopens_exact_durable_history", () => verifyRestart(context));
  await run("te03_independent_processes_preserve_exact_replay_after_restart", () => verifyRpcSurface(context));
  await run("ta02_actual_master_composition_receipt_history_restart_and_production_refusal", () => verifyMasterComposition(context));
  process.stdout.write(JSON.stringify({
    acceptance_complete: true, result: "PASS", cases: passed, public_processes: 2,
    signed_transactions: context.submitted.length, solidity_sources: contracts.size,
    master_compositions: 1,
    verification_mode: "LOCAL_DEV_UNAUTHENTICATED", authenticated_finality: false,
    bft_consensus_starvation: "NOT_IMPLEMENTED before B3/B6", native_lifecycle: "NOT_IMPLEMENTED before B5",
  }) + "\n");
} finally {
  clearTimeout(deadline);
  const stopped = await Promise.allSettled([...context.nodes, ...context.extraNodes].map(stopDevelopmentNode));
  const failures = stopped.filter((result) => result.status === "rejected");
  assert.deepEqual(failures, [], "Every task-owned public child must be stopped and waited");
}

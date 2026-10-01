// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import path from "node:path";
import type { AcceptanceContext, DevelopmentNode } from "../types/acceptance_types.js";

export async function startDevelopmentNode(context: AcceptanceContext, dataPath: string): Promise<DevelopmentNode> {
  const child = spawn(context.publicBinary, [
    "serve-dev", "--root", context.repositoryRoot, "--data", dataPath, "--genesis", context.genesisPath,
    "--mode", "DEV_ALL_IN_ONE", "--acknowledge-unsafe-development", "--http-address", "127.0.0.1:0",
    "--ws-address", "127.0.0.1:0", "--block-interval-ms", "100",
  ], { cwd: context.repositoryRoot, stdio: ["ignore", "pipe", "pipe"] });
  let stderr = "";
  child.stderr?.on("data", (data: Buffer) => { stderr = (stderr + data.toString()).slice(-65_536); });
  try {
    const startup = await new Promise<Record<string, unknown>>((resolve, reject) => {
      let buffer = "";
      const timer = setTimeout(() => reject(new Error("Public development startup timed out")), 30_000);
      child.on("error", (error) => { clearTimeout(timer); reject(error); });
      child.on("exit", (code) => { clearTimeout(timer); reject(new Error(`Public startup exited ${code}: ${stderr}`)); });
      child.stdout?.on("data", (data: Buffer) => {
        buffer += data.toString();
        if (buffer.length > 65_536) { clearTimeout(timer); reject(new Error("Startup output exceeds bound")); return; }
        const line = buffer.split("\n").find((candidate) => candidate.trim().startsWith("{"));
        if (!line || !buffer.includes("\n")) return;
        try { const parsed = JSON.parse(line) as Record<string, unknown>; clearTimeout(timer); resolve(parsed); }
        catch (error) { clearTimeout(timer); reject(error); }
      });
    });
    assert.equal(startup.verification_mode, "LOCAL_DEV_UNAUTHENTICATED");
    assert.equal(startup.authenticated_finality, false);
    assert.match(String(startup.http_address), /^127\.0\.0\.1:\d+$/);
    assert.match(String(startup.ws_address), /^127\.0\.0\.1:\d+$/);
    return { process: child, httpUrl: `http://${startup.http_address}`, wsUrl: `ws://${startup.ws_address}`, dataPath, startup,
      stderr: () => stderr, diagnosticPath: path.join(context.runDirectory, path.basename(dataPath) + "-stderr.log"),
    };
  } catch (error) {
    if (child.pid !== undefined && child.exitCode === null && child.signalCode === null) {
      await new Promise<void>((resolve) => { child.once("exit", () => resolve()); child.kill("SIGKILL"); });
    }
    throw error;
  }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import assert from "node:assert/strict";
import { writeFile } from "node:fs/promises";
import type { DevelopmentNode } from "../types/acceptance_types.js";

export async function stopDevelopmentNode(node: DevelopmentNode): Promise<void> {
  if (node.process.exitCode !== null || node.process.signalCode !== null) {
    await writeFile(node.diagnosticPath, node.stderr());
    return;
  }
  await new Promise<void>((resolve, reject) => {
    let timedOut = false;
    const timer = setTimeout(() => { timedOut = true; node.process.kill("SIGKILL"); }, 10_000);
    node.process.once("exit", () => {
      clearTimeout(timer);
      if (timedOut) reject(new Error("Task-owned public child required termination after its shutdown deadline"));
      else resolve();
    });
    assert.equal(node.process.kill("SIGTERM"), true);
  });
  await writeFile(node.diagnosticPath, node.stderr());
}

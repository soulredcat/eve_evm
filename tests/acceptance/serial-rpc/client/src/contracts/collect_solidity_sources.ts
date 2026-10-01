// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import { readdir, readFile } from "node:fs/promises";
import path from "node:path";

export async function collectSoliditySources(directory: string, prefix = ""): Promise<Record<string, { content: string }>> {
  const sources: Record<string, { content: string }> = {};
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    if (entry.isSymbolicLink()) throw new Error("Contract source symlinks are forbidden");
    const relative = prefix + entry.name;
    const absolute = path.join(directory, entry.name);
    if (entry.isDirectory()) Object.assign(sources, await collectSoliditySources(absolute, relative + "/"));
    else if (entry.name.endsWith(".sol")) sources[relative] = { content: await readFile(absolute, "utf8") };
    else throw new Error(`Unexpected contract source: ${relative}`);
  }
  return sources;
}

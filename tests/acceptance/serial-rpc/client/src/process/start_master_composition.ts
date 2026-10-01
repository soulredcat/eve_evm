// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

import type { AcceptanceContext, DevelopmentNode } from "../types/acceptance_types.js";
import { startDevelopmentNode } from "./start_development_node.js";

export async function startMasterComposition(context: AcceptanceContext, dataPath: string): Promise<DevelopmentNode> {
  // Exercise the real master entry point composing canonical public behavior.
  // No test consumer imports private master implementation or signing authority.
  return startDevelopmentNode({ ...context, publicBinary: context.masterBinary }, dataPath);
}

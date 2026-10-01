// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::verification::types::manifest_types::GateRegistry;
use anyhow::{Result, ensure};
use std::collections::BTreeSet;

/// Preserve the complete mandatory queue and its dependency contract.
pub fn validate_gate_catalog(registry: &GateRegistry) -> Result<()> {
    let expected = (0..=11)
        .map(|index| format!("B{index}"))
        .chain(["SEC0", "SEC1", "SEC3"].map(String::from))
        .collect::<BTreeSet<_>>();
    let actual = registry
        .gates
        .iter()
        .map(|gate| gate.id.clone())
        .collect::<BTreeSet<_>>();
    ensure!(
        registry.version == 1 && registry.gates.len() == expected.len() && actual == expected,
        "Mandatory EVE B0-B11/SEC0/SEC1/SEC3 core catalog is missing, duplicate or substituted"
    );
    for gate in &registry.gates {
        let required: &[&str] = match gate.id.as_str() {
            "B0" | "SEC0" => &[],
            "B1" => &["B0"],
            "B2" => &["B1"],
            "B3" => &["B2"],
            "B4" => &["B3"],
            "B5" => &["B1", "B2", "B3"],
            "B6" => &["B3", "B4"],
            "B7" => &["B2", "B3", "B4"],
            "B8" => &["B4", "B5", "B6"],
            "B9" => &["B6", "B7", "B8"],
            "B10" => &["B9", "SEC3"],
            "B11" => &["B10"],
            "SEC1" => &["B2", "B3", "B4", "B5", "B6"],
            "SEC3" => &["B8", "B9", "SEC1"],
            _ => unreachable!("the mandatory gate catalog was checked"),
        };
        let prerequisites = gate
            .prerequisites
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        ensure!(
            prerequisites.len() == gate.prerequisites.len()
                && !prerequisites.contains(gate.id.as_str())
                && prerequisites.iter().all(|id| actual.contains(*id)),
            "{} has duplicate, self or unknown prerequisite edges",
            gate.id
        );
        ensure!(
            required.iter().all(|id| prerequisites.contains(id)),
            "{} omits a required prerequisite",
            gate.id
        );
    }
    let mut completed = BTreeSet::new();
    while completed.len() < registry.gates.len() {
        let ready = registry
            .gates
            .iter()
            .filter(|gate| {
                !completed.contains(gate.id.as_str())
                    && gate
                        .prerequisites
                        .iter()
                        .all(|id| completed.contains(id.as_str()))
            })
            .map(|gate| gate.id.as_str())
            .collect::<Vec<_>>();
        ensure!(
            !ready.is_empty(),
            "Mandatory gate prerequisite graph contains a cycle"
        );
        completed.extend(ready);
    }
    Ok(())
}

use crate::{
    structure::inspection::resolve_source_path::resolve_source_path,
    verification::types::manifest_types::GateRegistry,
};
use anyhow::{Result, ensure};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CoreMapping {
    version: u32,
    cases: Vec<CoreRequirement>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CoreRequirement {
    id: String,
    expected: String,
    owner_gates: Vec<String>,
    required_tests: Vec<String>,
    foundation_groups: Vec<String>,
    foundation_scope: String,
    status: String,
}

/// Validate full requirements while keeping the B0 subset distinct from acceptance.
pub fn validate_core_mapping(
    root: &Path,
    registry: &GateRegistry,
) -> Result<BTreeMap<String, String>> {
    super::validate_gate_catalog::validate_gate_catalog(registry)?;
    let mapping: CoreMapping = toml::from_str(&std::fs::read_to_string(resolve_source_path(
        root,
        "config/gates/requirements/core.toml",
    )?)?)?;
    let expected_ids = (1..=12)
        .map(|index| format!("R{index:02}"))
        .collect::<BTreeSet<_>>();
    let actual_ids = mapping
        .cases
        .iter()
        .map(|case| case.id.clone())
        .collect::<BTreeSet<_>>();
    ensure!(
        mapping.version == 1 && mapping.cases.len() == 12 && actual_ids == expected_ids,
        "Missing, duplicate or substituted R01-R12 mapping"
    );
    let mut status = BTreeMap::new();
    for case in mapping.cases {
        ensure!(
            case.status == "NOT_ACHIEVED"
                && !case.expected.trim().is_empty()
                && !case.foundation_scope.trim().is_empty(),
            "{}: B0 foundation mapping cannot claim full requirement acceptance",
            case.id
        );
        let owners = case
            .owner_gates
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        ensure!(
            !owners.is_empty()
                && owners.len() == case.owner_gates.len()
                && owners
                    .iter()
                    .all(|id| registry.gates.iter().any(|gate| gate.id == *id)),
            "{} has missing/duplicate/unknown owning bulks",
            case.id
        );
        let required_owners: &[&str] = match case.id.as_str() {
            "R01" => &["B3", "SEC1"],
            "R02" => &["B6", "B9"],
            "R03" => &["B2", "B6"],
            "R04" => &["B1", "B3", "B4"],
            "R05" | "R06" => &["B4", "B6"],
            "R07" => &["B2", "B5"],
            "R08" => &["B7"],
            "R09" => &["B3", "B4", "B6", "B8"],
            "R10" => &["B8", "SEC1", "SEC3"],
            "R11" => &["B0", "B9"],
            "R12" => &["B10", "B11", "SEC3"],
            _ => unreachable!("the complete requirement catalog was checked"),
        };
        ensure!(
            required_owners.iter().all(|id| owners.contains(id)),
            "{} omits a required owning bulk",
            case.id
        );
        let families: &[(&str, u8, u8)] = match case.id.as_str() {
            "R01" => &[("T-C", 1, 10)],
            "R03" => &[("T-E", 1, 6), ("T-A", 1, 6)],
            "R04" => &[("T-S", 5, 7)],
            "R05" => &[("T-N", 1, 5)],
            "R07" => &[("T-V", 1, 10)],
            "R10" => &[("T-Q", 1, 8)],
            _ => &[],
        };
        let additional: &[&str] = match case.id.as_str() {
            "R01" => &["T-G08"],
            "R02" => &[
                "T-Q04",
                "CLEAN_COPY_PUBLIC_BUILD_RUN",
                "CLEAN_COPY_VALIDATOR_BUILD_RUN",
            ],
            "R03" => &["T-S01", "T-S02"],
            "R04" => &["T-C06", "T-G04", "T-G05", "T-N09", "T-N10"],
            "R05" => &["T-S04", "T-S08", "T-C08", "T-C09", "T-N10", "T-N11"],
            "R06" => &[
                "T-N06", "T-N08", "T-N09", "T-N11", "T-A04", "T-A05", "T-Q03",
            ],
            "R07" => &["T-E04"],
            "R08" => &[
                "SERIAL_PARALLEL_DIFFERENTIAL",
                "HOT_STATE_AND_SYSTEM_DIFFERENTIAL",
            ],
            "R09" => &["T-C03", "T-N05", "T-N07", "T-N10", "T-N12", "T-G06"],
            "R10" => &["T-G07"],
            "R11" => &[
                "ALL_REGISTERED_SOFTWARE_GATES",
                "MANIFEST_VALIDATION",
                "CLEAN_CHECKOUT_GATE_REPRODUCTION",
            ],
            "R12" => &[
                "W0",
                "W1",
                "W2",
                "W3",
                "W4",
                "W5",
                "W6",
                "SUSTAINED_1M_FINALIZED_60_MIN",
                "SOAK_24_HOURS",
                "BOUNDED_BACKLOG",
                "POST_RUN_RECOVERY",
            ],
            _ => unreachable!("the complete requirement catalog was checked"),
        };
        let required = families
            .iter()
            .flat_map(|(prefix, start, end)| {
                (*start..=*end).map(move |index| format!("{prefix}{index:02}"))
            })
            .chain(additional.iter().map(|id| (*id).to_owned()))
            .collect::<BTreeSet<_>>();
        let declared = case.required_tests.iter().cloned().collect::<BTreeSet<_>>();
        ensure!(
            declared.len() == case.required_tests.len()
                && !declared.contains("")
                && required.is_subset(&declared),
            "{} omits or duplicates a required full test family",
            case.id
        );
        ensure!(
            !case.foundation_groups.is_empty(),
            "{} lacks its concrete foundation test mapping",
            case.id
        );
        let mut groups = BTreeSet::new();
        for path in &case.foundation_groups {
            ensure!(
                groups.insert(path),
                "{} duplicates a foundation group",
                case.id
            );
            super::load_test_group::load_test_group(root, path)?;
        }
        status.insert(case.id, case.status);
    }
    Ok(status)
}

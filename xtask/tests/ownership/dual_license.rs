// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{Fixture, annotation, notice};

const PREFIX: &str = "validator/components/consensus-comet/tests/fixtures/zip215-upstream/";
const EXPRESSION: &str = "Apache-2.0 AND BSD-3-Clause";

fn fixture(file: &str, license: &str) -> Fixture {
    let fixture = Fixture::new();
    for identifier in ["Apache-2.0", "BSD-3-Clause"] {
        let path = format!("LICENSES/{identifier}.txt");
        fixture.write(&path, "Synthetic upstream terms for checker tests\n");
        fixture.pin(&path, "Synthetic upstream terms for checker tests\n");
    }
    let path = format!("{PREFIX}{file}");
    fixture.write(&path, "Synthetic exact upstream policy input\n");
    fixture.pin(&path, "Synthetic exact upstream policy input\n");
    fixture.annotation(&path, license, "Novi and Oasis contributors");
    fixture
}

#[test]
fn zip215_exact_conjunction_requires_both_separate_license_texts() {
    let fixture = fixture("zip215_cases-upstream_speccheck_0.json", EXPRESSION);
    assert_eq!(fixture.pass().coverage.upstream, 1);
}

#[test]
fn zip215_conjunction_rejects_either_missing_component_text() {
    for identifier in ["Apache-2.0", "BSD-3-Clause"] {
        let fixture = fixture("zip215_cases-upstream_speccheck_0.json", EXPRESSION);
        std::fs::remove_file(fixture.root().join(format!("LICENSES/{identifier}.txt"))).unwrap();
        fixture.reject("missing or empty license text");
    }
}

#[test]
fn zip215_conjunction_rejects_either_empty_component_text() {
    for identifier in ["Apache-2.0", "BSD-3-Clause"] {
        let fixture = fixture("zip215_cases-upstream_speccheck_0.json", EXPRESSION);
        fixture.write(&format!("LICENSES/{identifier}.txt"), " \n");
        fixture.reject("missing or empty license text");
    }
}

#[test]
fn zip215_combined_filename_cannot_replace_separate_license_texts() {
    let fixture = fixture("zip215_cases-upstream_speccheck_0.json", EXPRESSION);
    fixture.write(
        "LICENSES/Apache-2.0 AND BSD-3-Clause.txt",
        "Wrong single file\n",
    );
    fixture.reject("unexpected, nested or unused license text");
}

#[test]
fn zip215_reversed_nested_or_unknown_license_expressions_are_rejected() {
    for expression in [
        "BSD-3-Clause AND Apache-2.0",
        "Apache-2.0 OR BSD-3-Clause",
        "(Apache-2.0 AND BSD-3-Clause)",
        "Apache-2.0 AND BSD-2-Clause",
        "Apache-2.0 AND BSD-3-Clause AND MIT",
        " Apache-2.0 AND BSD-3-Clause",
        "Apache-2.0  AND  BSD-3-Clause",
    ] {
        let fixture = fixture("zip215_cases-upstream_speccheck_0.json", expression);
        fixture.reject("Unsupported ownership license expression");
    }
}

#[test]
fn zip215_first_party_annotation_cannot_use_upstream_conjunction() {
    let fixture = Fixture::new();
    for identifier in ["Apache-2.0", "BSD-3-Clause"] {
        let path = format!("LICENSES/{identifier}.txt");
        fixture.write(&path, "Synthetic upstream terms for checker tests\n");
        fixture.pin(&path, "Synthetic upstream terms for checker tests\n");
    }
    fixture.write("config.json", "{}\n");
    fixture.reuse(
        &annotation("config.json", EXPRESSION, "2026 Redcat").replace(
            "Preserved upstream source and terms.",
            "Use requires prior written permission from Redcat.",
        ),
    );
    fixture.reject("contradictory first-party annotation");
}

#[test]
fn zip215_only_twelve_exact_vector_filenames_are_classified_upstream() {
    for index in 0..12 {
        fixture(
            &format!("zip215_cases-upstream_speccheck_{index}.json"),
            EXPRESSION,
        )
        .pass();
    }
    for file in [
        "zip215_cases-upstream_speccheck_12.json",
        "zip215_cases-upstream_speccheck_00.json",
        "nested/zip215_cases-upstream_speccheck_0.json",
        "arbitrary.json",
    ] {
        fixture(file, EXPRESSION).reject("contradictory first-party annotation");
    }
}

#[test]
fn zip215_readme_remains_first_party_and_cannot_be_reclassified_by_annotation() {
    let fixture = Fixture::new();
    let path = format!("{PREFIX}README.md");
    fixture.write(&path, &notice("html", "First-party provenance\n"));
    assert_eq!(fixture.pass().coverage.inline_first_party, 4);
    fixture.annotation(&path, EXPRESSION, "Novi and Oasis contributors");
    fixture.reject("contradictory first-party annotation");
}

#[test]
fn zip215_upstream_inputs_cannot_pass_without_exact_source_digest() {
    for file in [
        "zip215_cases-upstream_speccheck_0.json",
        "NOTICE",
        "LICENSE-NOVI",
        "LICENSE",
    ] {
        let license = match file {
            "LICENSE" => "BSD-3-Clause",
            "LICENSE-NOVI" => "Apache-2.0",
            _ => EXPRESSION,
        };
        let fixture = fixture(file, license);
        let policy =
            std::fs::read_to_string(fixture.root().join("config/structure-policy.toml")).unwrap();
        let retained = policy.rsplit_once("\n[[exclusions]]\n").unwrap().0;
        fixture.write("config/structure-policy.toml", retained);
        fixture.reject(&format!(
            "{PREFIX}{file}: known immutable upstream file lacks its reviewed exact digest"
        ));
    }
}

#[test]
fn zip215_notice_and_original_license_files_preserve_their_distinct_licenses() {
    for (file, license) in [
        ("NOTICE", EXPRESSION),
        ("LICENSE", "BSD-3-Clause"),
        ("LICENSE-NOVI", "Apache-2.0"),
    ] {
        let fixture = fixture(file, license);
        let unused = if license == "BSD-3-Clause" {
            Some("Apache-2.0")
        } else if license == "Apache-2.0" {
            Some("BSD-3-Clause")
        } else {
            None
        };
        if let Some(identifier) = unused {
            std::fs::remove_file(fixture.root().join(format!("LICENSES/{identifier}.txt")))
                .unwrap();
        }
        fixture.pass();
        fixture.reuse(&annotation(
            &format!("{PREFIX}{file}"),
            "LicenseRef-Redcat-Permission-Only",
            "2026 Redcat",
        ));
        fixture.reject("Redcat ownership or changed license");
    }
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{Fixture, REDCAT, notice};

const UPSTREAM: &str =
    "validator/components/consensus-comet/vendor/cometbft/proto/tendermint/crypto/keys.proto";

fn fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(UPSTREAM, "syntax = \"proto3\";\n");
    fixture.pin(UPSTREAM, "syntax = \"proto3\";\n");
    fixture.write("LICENSES/Apache-2.0.txt", "Preserved upstream terms\n");
    fixture.pin("LICENSES/Apache-2.0.txt", "Preserved upstream terms\n");
    fixture.annotation(UPSTREAM, "Apache-2.0", "CometBFT contributors");
    fixture
}

#[test]
fn accepts_preserved_upstream_attribution_and_exact_bytes() {
    fixture().pass();
}

#[test]
fn rejects_redcat_ownership_on_upstream_annotation() {
    let fixture = fixture();
    fixture.annotation(UPSTREAM, REDCAT, "2026 Redcat");
    fixture.reject("Redcat ownership or changed license");
}

#[test]
fn rejects_redcat_notice_on_upstream_source() {
    let fixture = fixture();
    fixture.write(UPSTREAM, &notice("//", "syntax = \"proto3\";\n"));
    fixture.reject("Redcat ownership or changed license");
}

#[test]
fn rejects_missing_upstream_annotation() {
    let fixture = fixture();
    fixture.reuse("");
    fixture.reject("lacks its exact attribution annotation");
}

#[test]
fn rejects_upstream_byte_mutation() {
    let fixture = fixture();
    fixture.write(UPSTREAM, "syntax = \"proto2\";\n");
    fixture.reject("preserved upstream digest mismatch");
}

#[test]
fn rejects_missing_reviewed_upstream_digest() {
    let fixture = fixture();
    fixture.write(
        "config/structure-policy.toml",
        &notice("#", "version = 1\ncurrent_bulk = 0\n"),
    );
    fixture.reject("lacks its reviewed exact digest");
}

#[test]
fn rejects_redcat_notice_on_copied_upstream_license() {
    let fixture = fixture();
    fixture.write(
        "LICENSES/Apache-2.0.txt",
        "Copyright (c) 2026 Redcat\nPreserved terms\n",
    );
    fixture.reject("Redcat notice added to preserved upstream license text");
}

#[test]
fn rejects_redcat_permission_condition_in_upstream_metadata() {
    let fixture = fixture();
    let entries = super::support::annotation(UPSTREAM, "Apache-2.0", "CometBFT contributors")
        .replace(
            "Preserved upstream source and terms.",
            "Use requires prior written permission from Redcat.",
        );
    fixture.reuse(&entries);
    fixture.reject("Redcat ownership or changed license");
}

#[test]
fn rejects_metadata_contradicting_upstream_inline_license() {
    let fixture = fixture();
    let payload = "// SPDX-License-Identifier: BSD-3-Clause\nsyntax = \"proto3\";\n";
    fixture.write(UPSTREAM, payload);
    fixture.write(
        "config/structure-policy.toml",
        &notice("#", "version = 1\ncurrent_bulk = 0\n"),
    );
    fixture.pin(UPSTREAM, payload);
    fixture.pin("LICENSES/Apache-2.0.txt", "Preserved upstream terms\n");
    fixture.reject("contradicts preserved upstream inline license");
}

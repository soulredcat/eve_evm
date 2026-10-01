// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{Fixture, notice};

#[test]
fn accepts_formats_bom_crlf_shebang_and_rust_inner_attributes() {
    let fixture = Fixture::new();
    for (path, prefix, body) in [
        (
            "public/src/main.rs",
            "//",
            "#![forbid(unsafe_code)]\nfn main() {}\n",
        ),
        ("client.ts", "//", "export const x = 1;\n"),
        ("fixture.sol", "//", "pragma solidity =0.8.30;\n"),
        ("config.toml", "#", "version = 1\n"),
        ("workflow.yml", "#", "name: fixture\n"),
        ("guide.md", "html", "# Guide\n"),
        (".gitattributes", "#", "* text=auto eol=lf\n"),
    ] {
        fixture.write(
            path,
            &format!("\u{feff}{}", notice(prefix, body).replace('\n', "\r\n")),
        );
    }
    fixture.write(
        "executable.ts",
        &format!("#!/usr/bin/env node\n{}", notice("//", "export {};\n")),
    );
    fixture.pass();
}

#[test]
fn declaration_examples_are_not_false_license_conflicts() {
    let fixture = Fixture::new();
    fixture.write(
        "output.rs",
        &notice(
            "//",
            "pub const TEXT: &str = \"SPDX-License-Identifier: MIT\";\n",
        ),
    );
    fixture.write("guide.md", &notice("html", "<!-- REUSE-IgnoreStart -->\n<!-- SPDX-License-Identifier: MIT -->\n<!-- REUSE-IgnoreEnd -->\n"));
    fixture.pass();
}

#[test]
fn rejects_missing_inline_ownership() {
    let fixture = Fixture::new();
    fixture.write("public/src/main.rs", "fn main() {}\n");
    fixture.reject("missing exact three-line");
}

#[test]
fn rejects_contradictory_license_after_a_valid_notice() {
    let fixture = Fixture::new();
    fixture.write(
        "source.rs",
        &notice(
            "//",
            "// SPDX-License-Identifier: MIT\npub fn execute() {}\n",
        ),
    );
    fixture.reject("contradictory ownership declarations");
}

#[test]
fn rejects_duplicate_copyright_after_a_valid_notice() {
    let fixture = Fixture::new();
    fixture.write(
        "source.rs",
        &notice(
            "//",
            "// SPDX-FileCopyrightText: 2026 Redcat\npub fn execute() {}\n",
        ),
    );
    fixture.reject("duplicate or contradictory");
}

#[test]
fn rejects_missing_permission_sentence() {
    let fixture = Fixture::new();
    fixture.write(
        "source.rs",
        &notice("//", "pub fn execute() {}\n").replace(
            "// Use requires prior written permission from Redcat.\n",
            "",
        ),
    );
    fixture.reject("missing exact three-line");
}

#[test]
fn rejects_missing_blank_separator() {
    let fixture = Fixture::new();
    fixture.write(
        "source.rs",
        &notice("//", "pub fn execute() {}\n").replace("Redcat.\n\n", "Redcat.\n"),
    );
    fixture.reject("blank separator");
}

#[test]
fn rejects_unannotated_json_without_corrupting_bytes() {
    let fixture = Fixture::new();
    fixture.write("public/config.json", "{\"version\":1}\n");
    fixture.reject("unannotated noncommentable");
    assert_eq!(
        std::fs::read_to_string(fixture.root().join("public/config.json")).unwrap(),
        "{\"version\":1}\n"
    );
}

// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;
use std::path::Path;
use xtask::structure::{
    checking::check_structure::check_structure, discovery::discover_sources::discover_sources,
    policy::load_policy::load_policy, types::policy_types::StructurePolicy,
};

pub const FRAGMENT: &str = "config/structure/adapters.toml";
pub const ADAPTER: &str = "validator/src/codec_adapter.rs";

pub fn registration(path: &str) -> String {
    format!(
        "[[adapters]]\npath = \"{path}\"\nexternal_trait = \"upstream::Codec\"\nreason = \"External callbacks delegate to named operations.\"\nreviewer = \"test-reviewer\"\n"
    )
}

pub fn setup() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(ADAPTER, "impl upstream::Codec for Height {\n    fn encode(&self) { encoding::encode(self); }\n    fn decode(&self) { decoding::decode(self); }\n}\n");
    fixture.write(FRAGMENT, &format!("version = 1\n{}", registration(ADAPTER)));
    fixture.policy(&format!("adapter_files = [\"{FRAGMENT}\"]\n"));
    fixture
}

pub fn load(fixture: &Fixture) -> anyhow::Result<StructurePolicy> {
    let sources = discover_sources(fixture.root())?;
    load_policy(
        fixture.root(),
        Path::new("config/structure-policy.toml"),
        &sources,
    )
}

pub fn assert_load_rejected(fixture: &Fixture) {
    assert!(load(fixture).is_err(), "Invalid fragment was loaded");
    assert!(
        check_structure(fixture.root(), Path::new("config/structure-policy.toml")).is_err(),
        "Invalid fragment was accepted by the actual checker"
    );
}

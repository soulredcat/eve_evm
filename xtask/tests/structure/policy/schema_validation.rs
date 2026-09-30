use crate::support::Fixture;
use std::path::Path;
use xtask::structure::{
    checking::check_structure::check_structure, types::policy_types::StructurePolicy,
};

#[test]
fn unsupported_policy_version_and_bulk_fail_closed() {
    for policy in [
        "version = 2\ncurrent_bulk = 0\n",
        "version = 1\ncurrent_bulk = 12\n",
    ] {
        let fixture = Fixture::new();
        fixture.write("config/structure-policy.toml", policy);
        fixture.assert_rejected();
    }
}

#[test]
fn malformed_and_unknown_policy_fields_cannot_be_silently_ignored() {
    for policy in [
        "version = 1\ncurrent_bulk = 0\nignore_all_source = true\n",
        "version = 1\ncurrent_bulk = 0\n[[exceptions]]\npath = \"validator/src/encode_height.rs\"\nlines = 401\n",
        "version = 1\ncurrent_bulk = 0\n[[adapters]]\npath = \"validator/src/codec_adapter.rs\"\n",
        "version = \"one\"\ncurrent_bulk = 0\n",
    ] {
        assert!(toml::from_str::<StructurePolicy>(policy).is_err());
        let fixture = Fixture::new();
        fixture.write("config/structure-policy.toml", policy);
        assert!(
            check_structure(fixture.root(), Path::new("config/structure-policy.toml")).is_err()
        );
    }
}

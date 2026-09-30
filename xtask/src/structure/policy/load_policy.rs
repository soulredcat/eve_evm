use crate::structure::types::policy_types::StructurePolicy;
use anyhow::{Context, Result};
use std::path::Path;

pub fn load_policy(path: &Path) -> Result<StructurePolicy> {
    let encoded =
        std::fs::read_to_string(path).with_context(|| format!("Read policy {}", path.display()))?;
    toml::from_str(&encoded).context("Structure policy is invalid")
}

use super::validate_receipt_record;
use crate::provisioning::{
    execution::verify_tool_version,
    paths::resolve_contained_path,
    types::{ArtifactPin, ProvisionedArtifact},
};
use anyhow::{Result, ensure};
use std::path::Path;

pub fn load_verified_receipt(
    output: &Path,
    name: &str,
    pin: &ArtifactPin,
    recipe: &str,
    arguments: &[&str],
) -> Result<Option<ProvisionedArtifact>> {
    let path = resolve_contained_path(output, Path::new(&format!("{name}.receipt.json")))?;
    if !path.exists() {
        return Ok(None);
    }
    ensure!(
        std::fs::metadata(&path)?.len() <= 64 * 1024,
        "tool receipt exceeds limit"
    );
    let receipt: ProvisionedArtifact = serde_json::from_slice(&std::fs::read(path)?)?;
    validate_receipt_record(output, name, pin, recipe, &receipt)?;
    verify_tool_version(&receipt.executable, arguments, &pin.expected_version)?;
    Ok(Some(receipt))
}

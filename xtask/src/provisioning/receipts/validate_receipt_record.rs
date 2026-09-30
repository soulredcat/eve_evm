use crate::provisioning::{
    artifacts::compute_artifact_digest,
    paths::resolve_contained_path,
    types::{ArtifactPin, ProvisionedArtifact},
};
use anyhow::{Result, ensure};
use std::path::Path;

pub fn validate_receipt_record(
    output: &Path,
    name: &str,
    pin: &ArtifactPin,
    recipe: &str,
    receipt: &ProvisionedArtifact,
) -> Result<()> {
    let expected_path = resolve_contained_path(output, &Path::new(name).join(&pin.executable))?;
    ensure!(
        receipt.name == name
            && receipt.executable == expected_path
            && receipt.source_sha256 == pin.sha256
            && receipt.recipe_identity == recipe
            && receipt.version_output == pin.expected_version,
        "tool receipt does not match source/path/version/recipe pins"
    );
    let actual_digest = compute_artifact_digest(&expected_path)?;
    ensure!(
        actual_digest == receipt.executable_sha256,
        "existing executable changed after receipt"
    );
    if let Some(expected) = &pin.binary_sha256 {
        ensure!(
            actual_digest == *expected,
            "existing prebuilt binary differs from archive pin"
        );
    }
    Ok(())
}

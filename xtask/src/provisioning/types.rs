use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactPin {
    pub version: String,
    pub source_identity: String,
    pub url: String,
    pub sha256: String,
    pub archive_root: Option<String>,
    pub executable: String,
    pub binary_sha256: Option<String>,
    pub expected_version: String,
    pub license: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientPins {
    pub typescript: String,
    pub viem: String,
    pub manifest_sha256: String,
    pub lock_sha256: String,
    pub solana_kit: String,
    pub solana_kit_integrity: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolPins {
    pub version: u16,
    pub platform: String,
    pub go: ArtifactPin,
    pub comet: ArtifactPin,
    pub openssl: ArtifactPin,
    pub solidity: ArtifactPin,
    pub node: ArtifactPin,
    pub clients: ClientPins,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvisionedArtifact {
    pub name: String,
    pub executable: PathBuf,
    pub source_sha256: String,
    pub executable_sha256: String,
    pub version_output: String,
    pub recipe_identity: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvisionedTools {
    pub version: u16,
    pub pin_file_sha256: String,
    pub client_lock_sha256: String,
    pub client_tree_sha256: String,
    pub platform: String,
    pub artifacts: Vec<ProvisionedArtifact>,
    pub environment: BTreeMap<String, String>,
}

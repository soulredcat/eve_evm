use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCommitReference {
    pub height: u64,
    pub file_name: String,
    pub length: u64,
    pub sha256: [u8; 32],
    pub commit_identity: [u8; 32],
}

/// Complete local recovery export; checksums/root consistency are not source authentication.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalSnapshotManifest {
    pub format_version: u16,
    pub complete: bool,
    pub captured_sequence: u64,
    pub height: u64,
    pub head_commit_identity: [u8; 32],
    pub genesis_commit_identity: [u8; 32],
    pub total_bytes: u64,
    pub references: Vec<SnapshotCommitReference>,
}

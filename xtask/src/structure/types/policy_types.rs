use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructurePolicy {
    pub version: u32,
    pub current_bulk: u8,
    #[serde(default)]
    pub exclusions: Vec<Exclusion>,
    #[serde(default)]
    pub size_reviews: Vec<SizeReview>,
    #[serde(default)]
    pub exceptions: Vec<SizeException>,
    #[serde(default)]
    pub adapters: Vec<AdapterReview>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exclusion {
    pub path: String,
    pub kind: String,
    pub source: String,
    pub reason: String,
    pub generator: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SizeReview {
    pub path: String,
    pub lines: usize,
    pub reason: String,
    pub reviewer: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SizeException {
    pub path: String,
    pub lines: usize,
    pub reason: String,
    pub reviewer: String,
    pub split_task: String,
    pub expires_bulk: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterReview {
    pub path: String,
    pub external_trait: String,
    pub reason: String,
    pub reviewer: String,
}

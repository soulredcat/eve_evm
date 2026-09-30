use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct StructureReport {
    pub policy_version: u32,
    pub current_bulk: u8,
    pub files: Vec<FileReport>,
    pub exclusions: Vec<String>,
    pub warnings: Vec<String>,
    pub violations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FileReport {
    pub path: String,
    pub kind: String,
    pub physical_lines: usize,
    pub operations: Vec<String>,
    pub violations: Vec<String>,
}

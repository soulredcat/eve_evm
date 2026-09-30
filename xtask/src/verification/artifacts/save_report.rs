use crate::verification::types::report_types::VerificationReport;
use anyhow::Result;
use std::path::Path;

pub fn save_report(directory: &Path, report: &VerificationReport) -> Result<()> {
    std::fs::write(
        directory.join("report.json"),
        serde_json::to_vec_pretty(report)?,
    )?;
    Ok(())
}

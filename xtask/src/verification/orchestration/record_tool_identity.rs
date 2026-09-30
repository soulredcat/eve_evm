use crate::verification::types::report_types::VerificationReport;
use anyhow::Result;
use std::path::Path;

pub(super) fn record_tool_identity(root: &Path, report: &mut VerificationReport) -> Result<()> {
    let tools = crate::provisioning::validate_provisioned_tools(
        root,
        Path::new("config/tool-pins.toml"),
        Path::new("local-tests/toolchain-b0/provisioned-tools.json"),
    )?;
    report.tool_environment = tools.environment.clone();
    report.tool_evidence = Some(serde_json::to_value(tools)?);
    Ok(())
}

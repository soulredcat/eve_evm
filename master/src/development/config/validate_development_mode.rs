use anyhow::{Result, ensure};

pub fn validate_development_mode(mode: &str, acknowledged: bool) -> Result<()> {
    ensure!(
        mode == "DEV_ALL_IN_ONE" && acknowledged,
        "local producer requires DEV_ALL_IN_ONE and explicit development acknowledgement; MASTER_SYNC_ONLY/production cannot activate it"
    );
    Ok(())
}

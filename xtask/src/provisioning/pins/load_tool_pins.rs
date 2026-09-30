use super::validate_artifact_pin;
use crate::provisioning::{paths::resolve_contained_path, types::ToolPins};
use anyhow::{Result, ensure};
use std::path::Path;

pub fn load_tool_pins(root: &Path, path: &Path) -> Result<ToolPins> {
    let path = resolve_contained_path(root, path)?;
    ensure!(
        std::fs::metadata(&path)?.len() <= 64 * 1024,
        "tool pin file exceeds limit"
    );
    let pins: ToolPins = toml::from_str(&std::fs::read_to_string(path)?)?;
    ensure!(
        pins.version == 1 && pins.platform == "linux-x86_64",
        "unsupported tool pin schema/platform"
    );
    for pin in [
        &pins.go,
        &pins.comet,
        &pins.openssl,
        &pins.solidity,
        &pins.node,
    ] {
        validate_artifact_pin(pin)?;
    }
    ensure!(
        pins.go.archive_root.is_some()
            && pins.comet.archive_root.is_some()
            && pins.openssl.archive_root.is_some()
            && pins.node.archive_root.is_some()
            && pins.solidity.archive_root.is_none(),
        "unexpected artifact layout"
    );
    for digest in [&pins.clients.manifest_sha256, &pins.clients.lock_sha256] {
        ensure!(
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()),
            "client fixture requires exact source digests"
        );
    }
    Ok(pins)
}

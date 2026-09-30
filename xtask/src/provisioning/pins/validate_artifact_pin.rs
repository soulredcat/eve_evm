use crate::provisioning::types::ArtifactPin;
use anyhow::{Result, ensure};
use std::path::{Component, Path};

pub fn validate_artifact_pin(pin: &ArtifactPin) -> Result<()> {
    ensure!(
        pin.url.starts_with("https://"),
        "artifact URL requires HTTPS"
    );
    let host = pin.url[8..].split('/').next().unwrap_or_default();
    ensure!(
        [
            "go.dev",
            "dl.google.com",
            "codeload.github.com",
            "github.com",
            "nodejs.org"
        ]
        .contains(&host),
        "artifact host is outside the reviewed source allowlist"
    );
    ensure!(
        pin.sha256.len() == 64
            && pin
                .sha256
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()),
        "source SHA256 must be lowercase full-width hex"
    );
    if let Some(digest) = &pin.binary_sha256 {
        ensure!(
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()),
            "binary SHA256 must be lowercase full-width hex"
        );
    }
    ensure!(
        !pin.version.is_empty()
            && !pin.source_identity.is_empty()
            && !pin.expected_version.is_empty()
            && !pin.license.is_empty(),
        "tool source/version/license declaration is incomplete"
    );
    ensure!(
        !pin.executable.is_empty()
            && Path::new(&pin.executable)
                .components()
                .all(|part| matches!(part, Component::Normal(_))),
        "tool executable must be a contained relative path"
    );
    if let Some(root) = &pin.archive_root {
        ensure!(
            Path::new(root).components().count() == 1
                && Path::new(root)
                    .components()
                    .all(|part| matches!(part, Component::Normal(_))),
            "archive root must be one ordinary path component"
        );
    }
    Ok(())
}

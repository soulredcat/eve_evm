use super::source_edges::types::SourceTarget;
use anyhow::{Result, bail};
use std::path::Path;

pub fn manifest_source_targets(manifest: &Path) -> Result<Vec<SourceTarget>> {
    let document: toml::Value = toml::from_str(&std::fs::read_to_string(manifest)?)?;
    let parent = manifest.parent().expect("manifest parent");
    let mut targets = Vec::new();
    if let Some(path) = document.get("lib").and_then(|target| target.get("path")) {
        let Some(path) = path.as_str() else {
            bail!("Library source path must be a string");
        };
        targets.push(SourceTarget {
            path: parent.join(path),
            test_only: false,
        });
    }
    for kind in ["bin", "test", "example", "bench"] {
        if let Some(entries) = document.get(kind).and_then(toml::Value::as_array) {
            for entry in entries {
                if let Some(path) = entry.get("path") {
                    let Some(path) = path.as_str() else {
                        bail!("{kind} source path must be a string");
                    };
                    targets.push(SourceTarget {
                        path: parent.join(path),
                        test_only: kind == "test",
                    });
                }
            }
        }
    }
    if let Some(build) = document
        .get("package")
        .and_then(|package| package.get("build"))
    {
        if let Some(path) = build.as_str() {
            targets.push(SourceTarget {
                path: parent.join(path),
                test_only: false,
            });
        } else if build.as_bool().is_none() {
            bail!("Build source must be a string or boolean");
        }
    }
    Ok(targets)
}

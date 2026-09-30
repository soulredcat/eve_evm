use std::path::{Path, PathBuf};

pub fn source_package_root(root: &Path, source: &Path) -> PathBuf {
    if source.parent() == Some(root) {
        return root.to_path_buf();
    }
    let mut directory = source.parent();
    while let Some(candidate) = directory {
        if candidate.join("Cargo.toml").is_file() && candidate != root {
            return candidate.to_path_buf();
        }
        if candidate == root {
            break;
        }
        directory = candidate.parent();
    }
    let role = source
        .strip_prefix(root)
        .ok()
        .and_then(|relative| relative.components().next());
    role.map_or_else(
        || root.to_path_buf(),
        |segment| root.join(segment.as_os_str()),
    )
}

use super::{
    classify_file::classify_file, count_physical_lines::count_physical_lines,
    review_size::review_size,
};
use crate::structure::{
    syntax::inspect_rust_syntax::inspect_rust_syntax,
    types::{
        policy_types::StructurePolicy, report_types::FileReport, syntax_types::SyntaxInventory,
    },
};
use anyhow::{Context, Result, bail};
use std::path::Path;

pub fn inspect_file(
    root: &Path,
    path: &str,
    policy: &StructurePolicy,
) -> Result<(FileReport, Vec<String>)> {
    let absolute = root.join(path);
    let metadata =
        std::fs::symlink_metadata(&absolute).with_context(|| format!("Inspect {path}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        bail!("First-party source must be a regular file: {path}");
    }
    let source =
        std::fs::read_to_string(&absolute).with_context(|| format!("Read UTF-8 source {path}"))?;
    let physical_lines = count_physical_lines(&source);
    let inventory = if path.ends_with(".rs") {
        inspect_rust_syntax(&source)?
    } else {
        SyntaxInventory::default()
    };
    let kind = classify_file(path, policy, inventory.operations.len());
    let (warnings, mut violations) = review_size(path, physical_lines, policy);
    if path.starts_with("crates/") || path.starts_with("create/") {
        violations.push(format!(
            "{path}: root shared-code directories violate absolute role placement"
        ));
    }
    match kind {
        "unsupported" => violations.push(format!(
            "{path}: unsupported source category; implement explicit coverage"
        )),
        "facade" if !inventory.operations.is_empty() => {
            violations.push(format!("{path}: facade contains behavior"))
        }
        "entry" if inventory.operations != ["main"] => {
            violations.push(format!("{path}: entry must own only main"))
        }
        "adapter" if physical_lines > 200 || inventory.nondelegating_methods > 0 => violations
            .push(format!(
                "{path}: adapter is oversized or contains nondelegating behavior"
            )),
        "behavior" => {
            if inventory.operations.len() != 1 {
                violations.push(format!(
                    "{path}: expected one primary operation, found {}",
                    inventory.operations.len()
                ));
            } else if Path::new(path).file_stem().and_then(|stem| stem.to_str())
                != inventory.operations.first().map(String::as_str)
            {
                violations.push(format!(
                    "{path}: file must be named after {}",
                    inventory.operations[0]
                ));
            }
        }
        _ => {}
    }
    let stem = Path::new(path)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    if matches!(
        stem,
        "utils" | "helpers" | "common" | "shared" | "misc" | "manager"
    ) || stem.strip_prefix("part").is_some_and(|suffix| {
        !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
    }) {
        violations.push(format!("{path}: catch-all or opaque split is forbidden"));
    }
    if kind != "test" && (inventory.opaque_macros > 0 || inventory.complex_closures > 0) {
        violations.push(format!("{path}: macro/closure hides an operation"));
    }
    Ok((
        FileReport {
            path: path.into(),
            kind: kind.into(),
            physical_lines,
            operations: inventory.operations,
            violations,
        },
        warnings,
    ))
}

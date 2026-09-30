use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

pub fn validate_role_dependencies(root: &Path, sources: &[String]) -> Vec<String> {
    let mut violations = Vec::new();
    let Ok(root) = root.canonicalize() else {
        return vec!["Cannot resolve role dependency root".into()];
    };
    let mut graph: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for source in sources
        .iter()
        .filter(|source| source.ends_with("Cargo.toml"))
    {
        let manifest = root.join(source);
        match super::dependency_targets::dependency_targets(&root, &manifest) {
            Ok(targets) => {
                graph.insert(manifest.canonicalize().unwrap_or(manifest), targets);
            }
            Err(error) => violations.push(format!(
                "{source}: invalid role dependency graph: {error:#}"
            )),
        }
    }
    for (manifest, targets) in &graph {
        for target in targets {
            if !graph.contains_key(target) {
                violations.push(format!(
                    "{}: local dependency manifest is missing from reviewed source inventory: {}",
                    manifest.display(),
                    target.display()
                ));
            }
        }
    }
    for start in graph.keys().filter(|path| {
        path.starts_with(root.join("public")) || path.starts_with(root.join("validator"))
    }) {
        let mut pending = vec![start.clone()];
        let mut visited = BTreeSet::new();
        while let Some(current) = pending.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            if current.starts_with(root.join("master")) {
                violations.push(format!(
                    "{}: direct/transitive private master dependency is forbidden: {}",
                    start.display(),
                    current.display()
                ));
                break;
            }
            if let Some(children) = graph.get(&current) {
                pending.extend(children.iter().cloned());
            }
        }
    }
    violations.extend(super::validate_source_edges::validate_source_edges(
        &root, sources,
    ));
    violations
}

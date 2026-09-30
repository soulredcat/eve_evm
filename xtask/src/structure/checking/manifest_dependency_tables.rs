use toml::value::Table;

pub fn manifest_dependency_tables(document: &toml::Value) -> Vec<&Table> {
    let mut tables = Vec::new();
    for kind in [
        "dependencies",
        "build-dependencies",
        "dev-dependencies",
        "replace",
    ] {
        if let Some(table) = document.get(kind).and_then(toml::Value::as_table) {
            tables.push(table);
        }
    }
    if let Some(targets) = document.get("target").and_then(toml::Value::as_table) {
        for target in targets.values() {
            for kind in ["dependencies", "build-dependencies", "dev-dependencies"] {
                if let Some(table) = target.get(kind).and_then(toml::Value::as_table) {
                    tables.push(table);
                }
            }
        }
    }
    if let Some(patches) = document.get("patch").and_then(toml::Value::as_table) {
        for patch in patches.values() {
            if let Some(table) = patch.as_table() {
                tables.push(table);
            }
        }
    }
    tables
}

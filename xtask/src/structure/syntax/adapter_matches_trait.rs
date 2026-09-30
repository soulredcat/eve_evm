use crate::structure::types::syntax_types::SyntaxInventory;
use std::collections::BTreeMap;

pub fn adapter_matches_trait(source: &str, inventory: &SyntaxInventory, expected: &str) -> bool {
    if inventory.free_operations > 0
        || inventory.inherent_impls > 0
        || inventory.implemented_traits.is_empty()
        || inventory.nondelegating_methods > 0
    {
        return false;
    }
    let Ok(file) = syn::parse_file(source) else {
        return false;
    };
    let mut imports = BTreeMap::new();
    for item in &file.items {
        if let syn::Item::Use(import) = item {
            super::collect_use_tree::collect_use_tree(&import.tree, "", &mut imports);
        }
    }
    inventory.implemented_traits.iter().all(|path| {
        path == expected
            || imports
                .get(path)
                .is_some_and(|resolved| resolved == expected)
    })
}

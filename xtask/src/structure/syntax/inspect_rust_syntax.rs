use super::syntax_inventory_adapter::SyntaxInventoryAdapter;
use crate::structure::types::syntax_types::SyntaxInventory;
use anyhow::{Context, Result};
use syn::visit::Visit;

pub fn inspect_rust_syntax(source: &str) -> Result<SyntaxInventory> {
    let file = syn::parse_file(source).context("Rust source could not be parsed")?;
    let mut inventory = SyntaxInventory::default();
    SyntaxInventoryAdapter {
        inventory: &mut inventory,
    }
    .visit_file(&file);
    Ok(inventory)
}

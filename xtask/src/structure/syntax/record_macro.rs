use super::syntax_inventory_adapter::SyntaxInventoryAdapter;

pub fn record_macro(visitor: &mut SyntaxInventoryAdapter<'_>, operation: &syn::Macro) {
    if operation.path.is_ident("include") {
        visitor.inventory.includes += 1;
        visitor.inventory.opaque_macros += 1;
    } else if !super::is_bounded_macro::is_bounded_macro(operation) {
        visitor.inventory.opaque_macros += 1;
    }
    syn::visit::visit_macro(visitor, operation);
}

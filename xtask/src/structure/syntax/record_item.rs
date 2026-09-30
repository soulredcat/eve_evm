use super::{
    has_test_configuration::has_test_configuration,
    syntax_inventory_adapter::SyntaxInventoryAdapter,
};

pub fn record_item(visitor: &mut SyntaxInventoryAdapter<'_>, item: &syn::Item) {
    match item {
        syn::Item::Fn(function) => {
            if has_test_configuration(&function.attrs) {
                return;
            }
            visitor
                .inventory
                .operations
                .push(function.sig.ident.to_string());
        }
        syn::Item::Mod(module) if has_test_configuration(&module.attrs) => return,
        syn::Item::Impl(implementation) if has_test_configuration(&implementation.attrs) => return,
        syn::Item::Macro(definition) if definition.ident.is_some() => {
            visitor.inventory.opaque_macros += 1;
        }
        _ => {}
    }
    syn::visit::visit_item(visitor, item);
}

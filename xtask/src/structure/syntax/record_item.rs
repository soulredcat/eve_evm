use super::{
    has_test_configuration::has_test_configuration,
    syntax_inventory_adapter::SyntaxInventoryAdapter,
};

pub fn record_item(visitor: &mut SyntaxInventoryAdapter<'_>, item: &syn::Item) {
    if has_test_configuration(super::item_attributes::item_attributes(item)) {
        return;
    }
    match item {
        syn::Item::Fn(function) => {
            if has_test_configuration(&function.attrs) {
                return;
            }
            visitor
                .inventory
                .operations
                .push(function.sig.ident.to_string());
            visitor.inventory.free_operations += 1;
        }
        syn::Item::Mod(module) if has_test_configuration(&module.attrs) => return,
        syn::Item::Impl(implementation) if has_test_configuration(&implementation.attrs) => return,
        syn::Item::Impl(implementation) => {
            if let Some((_, path, _)) = &implementation.trait_ {
                visitor.inventory.implemented_traits.push(
                    path.segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::"),
                );
            } else {
                visitor.inventory.inherent_impls += 1;
            }
        }
        syn::Item::Const(value) if !super::is_pure_constant::is_pure_constant(&value.expr) => {
            visitor.inventory.executable_initializers += 1;
        }
        syn::Item::Static(value) if !super::is_pure_constant::is_pure_constant(&value.expr) => {
            visitor.inventory.executable_initializers += 1;
        }
        _ => {}
    }
    syn::visit::visit_item(visitor, item);
}

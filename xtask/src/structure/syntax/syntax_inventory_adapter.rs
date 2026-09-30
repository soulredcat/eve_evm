use super::{record_closure, record_impl_method, record_item, record_trait_method};
use crate::structure::types::syntax_types::SyntaxInventory;
use syn::visit::Visit;

pub struct SyntaxInventoryAdapter<'a> {
    pub inventory: &'a mut SyntaxInventory,
}

impl<'ast> Visit<'ast> for SyntaxInventoryAdapter<'_> {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        record_item::record_item(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        record_impl_method::record_impl_method(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        record_trait_method::record_trait_method(self, node);
    }

    fn visit_expr_closure(&mut self, node: &'ast syn::ExprClosure) {
        record_closure::record_closure(self, node);
    }
}

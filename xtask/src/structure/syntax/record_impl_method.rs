// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    has_test_configuration::has_test_configuration, is_thin_delegate::is_thin_delegate,
    syntax_inventory_adapter::SyntaxInventoryAdapter,
};

pub fn record_impl_method(visitor: &mut SyntaxInventoryAdapter<'_>, method: &syn::ImplItemFn) {
    if has_test_configuration(&method.attrs) {
        return;
    }
    visitor
        .inventory
        .operations
        .push(method.sig.ident.to_string());
    if !is_thin_delegate(&method.block) {
        visitor.inventory.nondelegating_methods += 1;
    }
    syn::visit::visit_impl_item_fn(visitor, method);
}

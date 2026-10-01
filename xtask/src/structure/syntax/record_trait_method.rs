// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    has_test_configuration::has_test_configuration, is_thin_delegate::is_thin_delegate,
    syntax_inventory_adapter::SyntaxInventoryAdapter,
};

pub fn record_trait_method(visitor: &mut SyntaxInventoryAdapter<'_>, method: &syn::TraitItemFn) {
    if has_test_configuration(&method.attrs) {
        return;
    }
    if let Some(body) = &method.default {
        visitor
            .inventory
            .operations
            .push(method.sig.ident.to_string());
        if !is_thin_delegate(body) {
            visitor.inventory.nondelegating_methods += 1;
        }
    }
    syn::visit::visit_trait_item_fn(visitor, method);
}

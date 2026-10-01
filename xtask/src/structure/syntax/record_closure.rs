// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::syntax_inventory_adapter::SyntaxInventoryAdapter;

pub fn record_closure(visitor: &mut SyntaxInventoryAdapter<'_>, closure: &syn::ExprClosure) {
    if super::closure_complexity::closure_complexity(&closure.body) > 3 {
        visitor.inventory.complex_closures += 1;
    }
    syn::visit::visit_expr_closure(visitor, closure);
}

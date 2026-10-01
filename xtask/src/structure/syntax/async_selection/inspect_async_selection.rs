// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::parse_select_arguments_adapter::SelectArguments;
use crate::structure::syntax::syntax_inventory_adapter::SyntaxInventoryAdapter;
use syn::visit::Visit;

/// An explicit Tokio macro is parsed, then visited as ordinary Rust AST. This
/// rejects hidden functions, opaque nested macros and complex closures normally.
pub fn inspect_async_selection(
    visitor: &mut SyntaxInventoryAdapter<'_>,
    operation: &syn::Macro,
) -> bool {
    let path: Vec<_> = operation
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    if path != ["tokio", "select"] || operation.tokens.to_string().len() > 65_536 {
        return false;
    }
    let Ok(arguments) = syn::parse2::<SelectArguments>(operation.tokens.clone()) else {
        return false;
    };
    for expression in &arguments.expressions {
        visitor.visit_expr(expression);
    }
    for pattern in &arguments.patterns {
        visitor.visit_pat(pattern);
    }
    true
}

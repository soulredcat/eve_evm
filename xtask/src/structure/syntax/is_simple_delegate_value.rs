// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub fn is_simple_delegate_value(expression: &syn::Expr) -> bool {
    match expression {
        syn::Expr::Path(_) | syn::Expr::Lit(_) => true,
        syn::Expr::Reference(value) => is_simple_delegate_value(&value.expr),
        syn::Expr::Field(value) => is_simple_delegate_value(&value.base),
        syn::Expr::Paren(value) => is_simple_delegate_value(&value.expr),
        syn::Expr::Group(value) => is_simple_delegate_value(&value.expr),
        syn::Expr::Unary(value) => is_simple_delegate_value(&value.expr),
        _ => false,
    }
}

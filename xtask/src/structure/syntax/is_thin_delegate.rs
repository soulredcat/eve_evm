// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub fn is_thin_delegate(block: &syn::Block) -> bool {
    let [syn::Stmt::Expr(expression, _)] = block.stmts.as_slice() else {
        return false;
    };
    let call = match expression {
        syn::Expr::Return(result) => result.expr.as_deref().unwrap_or(expression),
        syn::Expr::Try(result) => result.expr.as_ref(),
        _ => expression,
    };
    let arguments = match call {
        syn::Expr::Call(call) if matches!(call.func.as_ref(), syn::Expr::Path(_)) => &call.args,
        syn::Expr::MethodCall(call)
            if super::is_simple_delegate_value::is_simple_delegate_value(&call.receiver) =>
        {
            &call.args
        }
        _ => return false,
    };
    arguments
        .iter()
        .all(super::is_simple_delegate_value::is_simple_delegate_value)
}

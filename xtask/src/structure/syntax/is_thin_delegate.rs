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
        syn::Expr::Call(call) => &call.args,
        syn::Expr::MethodCall(call) => &call.args,
        _ => return false,
    };
    arguments.iter().all(|argument| {
        matches!(
            argument,
            syn::Expr::Path(_) | syn::Expr::Reference(_) | syn::Expr::Field(_) | syn::Expr::Lit(_)
        )
    })
}

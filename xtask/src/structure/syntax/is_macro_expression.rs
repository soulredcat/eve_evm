pub fn is_macro_expression(expression: &syn::Expr) -> bool {
    match expression {
        syn::Expr::Lit(_) | syn::Expr::Path(_) | syn::Expr::Infer(_) => true,
        syn::Expr::Paren(value) => is_macro_expression(&value.expr),
        syn::Expr::Group(value) => is_macro_expression(&value.expr),
        syn::Expr::Reference(value) => is_macro_expression(&value.expr),
        syn::Expr::Unary(value) => is_macro_expression(&value.expr),
        syn::Expr::Binary(value) => {
            is_macro_expression(&value.left) && is_macro_expression(&value.right)
        }
        syn::Expr::Cast(value) => is_macro_expression(&value.expr),
        syn::Expr::Field(value) => is_macro_expression(&value.base),
        syn::Expr::Index(value) => {
            is_macro_expression(&value.expr) && is_macro_expression(&value.index)
        }
        syn::Expr::Try(value) => is_macro_expression(&value.expr),
        syn::Expr::Array(value) => value.elems.iter().all(is_macro_expression),
        syn::Expr::Tuple(value) => value.elems.iter().all(is_macro_expression),
        syn::Expr::Repeat(value) => {
            is_macro_expression(&value.expr) && is_macro_expression(&value.len)
        }
        syn::Expr::Range(value) => {
            value.start.as_deref().is_none_or(is_macro_expression)
                && value.end.as_deref().is_none_or(is_macro_expression)
        }
        syn::Expr::Call(value) => {
            is_macro_expression(&value.func) && value.args.iter().all(is_macro_expression)
        }
        syn::Expr::MethodCall(value) => {
            is_macro_expression(&value.receiver) && value.args.iter().all(is_macro_expression)
        }
        syn::Expr::Macro(value) => super::is_bounded_macro::is_bounded_macro(&value.mac),
        syn::Expr::Closure(value) => value.asyncness.is_none() && is_macro_expression(&value.body),
        _ => false,
    }
}

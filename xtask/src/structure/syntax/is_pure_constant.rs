pub fn is_pure_constant(expression: &syn::Expr) -> bool {
    match expression {
        syn::Expr::Lit(_) | syn::Expr::Path(_) => true,
        syn::Expr::Array(value) => value.elems.iter().all(is_pure_constant),
        syn::Expr::Tuple(value) => value.elems.iter().all(is_pure_constant),
        syn::Expr::Struct(value) => {
            value
                .fields
                .iter()
                .all(|field| is_pure_constant(&field.expr))
                && value.rest.as_deref().is_none_or(is_pure_constant)
        }
        syn::Expr::Repeat(value) => is_pure_constant(&value.expr) && is_pure_constant(&value.len),
        syn::Expr::Paren(value) => is_pure_constant(&value.expr),
        syn::Expr::Group(value) => is_pure_constant(&value.expr),
        syn::Expr::Reference(value) => is_pure_constant(&value.expr),
        syn::Expr::Unary(value) => is_pure_constant(&value.expr),
        syn::Expr::Binary(value) => is_pure_constant(&value.left) && is_pure_constant(&value.right),
        syn::Expr::Cast(value) => is_pure_constant(&value.expr),
        syn::Expr::Macro(value) => {
            value.mac.path.segments.last().is_some_and(|segment| {
                matches!(
                    segment.ident.to_string().as_str(),
                    "address" | "b256" | "hex"
                )
            }) && syn::parse2::<syn::LitStr>(value.mac.tokens.clone()).is_ok()
        }
        _ => false,
    }
}

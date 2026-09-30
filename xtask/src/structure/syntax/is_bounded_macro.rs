use syn::{Expr, Token, parse::Parser, punctuated::Punctuated};

pub fn is_bounded_macro(operation: &syn::Macro) -> bool {
    if operation.path.segments.len() > 1
        && operation.path.segments.first().is_none_or(|segment| {
            !matches!(
                segment.ident.to_string().as_str(),
                "std" | "core" | "anyhow" | "alloy_primitives" | "syn"
            )
        })
    {
        return false;
    }
    let Some(name) = operation
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
    else {
        return false;
    };
    if name == "cfg" {
        return syn::parse2::<syn::Meta>(operation.tokens.clone())
            .is_ok_and(|predicate| super::is_cfg_predicate::is_cfg_predicate(&predicate));
    }
    if name == "Token" {
        return matches!(
            operation.tokens.to_string().as_str(),
            "," | ";"
                | "="
                | "=>"
                | "::"
                | "*"
                | "+"
                | "-"
                | "/"
                | "!"
                | "?"
                | "<"
                | ">"
                | "&"
                | "|"
                | ":"
                | "."
        );
    }
    if !matches!(
        name.as_str(),
        "format"
            | "format_args"
            | "print"
            | "println"
            | "eprint"
            | "eprintln"
            | "write"
            | "writeln"
            | "anyhow"
            | "bail"
            | "ensure"
            | "panic"
            | "unreachable"
            | "assert"
            | "assert_eq"
            | "assert_ne"
            | "debug_assert"
            | "debug_assert_eq"
            | "debug_assert_ne"
            | "matches"
            | "vec"
            | "env"
            | "option_env"
            | "concat"
            | "include_str"
            | "include_bytes"
            | "address"
            | "b256"
            | "hex"
    ) {
        return false;
    }
    if name == "vec" {
        return syn::parse_str::<Expr>(&format!("[{}]", operation.tokens))
            .is_ok_and(|expression| super::is_macro_expression::is_macro_expression(&expression));
    }
    Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(operation.tokens.clone())
        .is_ok_and(|expressions| {
            expressions
                .iter()
                .all(super::is_macro_expression::is_macro_expression)
        })
}

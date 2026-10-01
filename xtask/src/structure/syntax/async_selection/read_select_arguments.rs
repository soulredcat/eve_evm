// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::parse_select_arguments_adapter::SelectArguments;
use syn::{Expr, Ident, Pat, Result, Token, parse::ParseStream};

/// Parse only Tokio's select branch grammar, retaining every executable expression.
pub(super) fn read_select_arguments(input: ParseStream<'_>) -> Result<SelectArguments> {
    let mut expressions = Vec::new();
    let mut patterns = Vec::new();
    let mut branches = 0_usize;
    if input.peek(Ident) && input.peek2(Token![;]) {
        let marker: Ident = input.parse()?;
        if marker != "biased" {
            return Err(input.error("unsupported async-selection marker"));
        }
        input.parse::<Token![;]>()?;
    }
    while !input.is_empty() {
        branches += 1;
        if branches > 32 {
            return Err(input.error("async selection exceeds branch inspection bound"));
        }
        if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            input.parse::<Token![=>]>()?;
            expressions.push(input.parse::<Expr>()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            if !input.is_empty() {
                return Err(input.error("async selection else must be last"));
            }
            break;
        }
        patterns.push(Pat::parse_multi_with_leading_vert(input)?);
        input.parse::<Token![=]>()?;
        expressions.push(input.parse::<Expr>()?);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse::<Token![if]>()?;
            expressions.push(input.parse::<Expr>()?);
        }
        input.parse::<Token![=>]>()?;
        let body: Expr = input.parse()?;
        let permits_omitted_comma = matches!(&body, Expr::Block(_));
        expressions.push(body);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else if !input.is_empty() && !permits_omitted_comma {
            return Err(input.error("async selection branch needs a comma"));
        }
    }
    if branches == 0 {
        return Err(input.error("empty async selection"));
    }
    Ok(SelectArguments {
        expressions,
        patterns,
    })
}

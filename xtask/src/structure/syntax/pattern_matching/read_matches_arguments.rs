// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::parse_matches_arguments_adapter::MatchesArguments;
use syn::{Expr, Pat, Result, Token, parse::ParseStream};
pub(super) fn read_matches_arguments(input: ParseStream<'_>) -> Result<MatchesArguments> {
    let value = input.parse::<Expr>()?;
    input.parse::<Token![,]>()?;
    let pattern = Pat::parse_multi_with_leading_vert(input)?;
    let guard = if input.peek(Token![if]) {
        input.parse::<Token![if]>()?;
        Some(input.parse::<Expr>()?)
    } else {
        None
    };
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    }
    if !input.is_empty() {
        return Err(input.error("unexpected matches argument tokens"));
    }
    Ok(MatchesArguments {
        value,
        pattern,
        guard,
    })
}

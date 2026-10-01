// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use syn::{
    Expr, Pat, Result,
    parse::{Parse, ParseStream},
};
pub(super) struct MatchesArguments {
    pub value: Expr,
    pub pattern: Pat,
    pub guard: Option<Expr>,
}
impl Parse for MatchesArguments {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        super::read_matches_arguments::read_matches_arguments(input)
    }
}

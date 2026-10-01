// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use syn::{
    Expr, Pat, Result,
    parse::{Parse, ParseStream},
};

pub(super) struct SelectArguments {
    pub expressions: Vec<Expr>,
    pub patterns: Vec<Pat>,
}

/// Thin parser-trait entry; the named grammar operation owns all branch parsing.
impl Parse for SelectArguments {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        super::read_select_arguments::read_select_arguments(input)
    }
}

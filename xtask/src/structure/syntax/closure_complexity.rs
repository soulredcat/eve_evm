pub fn closure_complexity(expression: &syn::Expr) -> usize {
    match expression {
        syn::Expr::Block(value) => {
            value.block.stmts.len()
                + value
                    .block
                    .stmts
                    .iter()
                    .map(|statement| match statement {
                        syn::Stmt::Expr(expression, _) => closure_complexity(expression),
                        syn::Stmt::Local(local) => local
                            .init
                            .as_ref()
                            .map_or(0, |init| closure_complexity(&init.expr)),
                        syn::Stmt::Item(_) | syn::Stmt::Macro(_) => 1,
                    })
                    .sum::<usize>()
        }
        syn::Expr::If(value) => {
            closure_complexity(&value.cond)
                + closure_complexity(&syn::Expr::Block(syn::ExprBlock {
                    attrs: vec![],
                    label: None,
                    block: value.then_branch.clone(),
                }))
                + value
                    .else_branch
                    .as_ref()
                    .map_or(0, |(_, branch)| closure_complexity(branch))
        }
        syn::Expr::Match(value) => {
            value.arms.len()
                + value
                    .arms
                    .iter()
                    .map(|arm| closure_complexity(&arm.body))
                    .sum::<usize>()
        }
        syn::Expr::Loop(value) => 4 + value.body.stmts.len(),
        syn::Expr::While(value) => 4 + value.body.stmts.len(),
        syn::Expr::ForLoop(value) => 4 + value.body.stmts.len(),
        syn::Expr::Paren(value) => closure_complexity(&value.expr),
        syn::Expr::Group(value) => closure_complexity(&value.expr),
        syn::Expr::Return(value) => value
            .expr
            .as_ref()
            .map_or(0, |expression| closure_complexity(expression)),
        _ => 0,
    }
}

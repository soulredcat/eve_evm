use super::syntax_inventory_adapter::SyntaxInventoryAdapter;

pub fn record_closure(visitor: &mut SyntaxInventoryAdapter<'_>, closure: &syn::ExprClosure) {
    if let syn::Expr::Block(body) = closure.body.as_ref()
        && body.block.stmts.len() > 3
    {
        visitor.inventory.complex_closures += 1;
    }
    syn::visit::visit_expr_closure(visitor, closure);
}

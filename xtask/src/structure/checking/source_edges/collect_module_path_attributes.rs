use super::types::ModulePathAttributes;
use anyhow::{Result, bail};
use syn::{Meta, Token, punctuated::Punctuated};

pub fn collect_module_path_attributes(
    attributes: &[syn::Attribute],
) -> Result<ModulePathAttributes> {
    let mut pending = attributes
        .iter()
        .map(|attribute| (attribute.meta.clone(), false))
        .collect::<Vec<_>>();
    let mut paths = ModulePathAttributes::default();
    while let Some((meta, conditional)) = pending.pop() {
        match meta {
            Meta::NameValue(value) if value.path.is_ident("path") => {
                let syn::Expr::Lit(literal) = value.value else {
                    bail!("Module source path must be a literal");
                };
                let syn::Lit::Str(path) = literal.lit else {
                    bail!("Module source path must be a string literal");
                };
                paths.paths.push(path.value());
                paths.has_unconditional_path |= !conditional;
            }
            Meta::List(list) if list.path.is_ident("cfg_attr") => {
                let alternatives =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                pending.extend(alternatives.into_iter().skip(1).map(|meta| (meta, true)));
            }
            _ => {}
        }
    }
    Ok(paths)
}

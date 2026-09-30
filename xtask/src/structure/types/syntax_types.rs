#[derive(Debug, Default)]
pub struct SyntaxInventory {
    pub operations: Vec<String>,
    pub opaque_macros: usize,
    pub complex_closures: usize,
    pub nondelegating_methods: usize,
    pub free_operations: usize,
    pub inherent_impls: usize,
    pub implemented_traits: Vec<String>,
    pub executable_initializers: usize,
    pub includes: usize,
}

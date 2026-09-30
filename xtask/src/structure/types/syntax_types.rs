#[derive(Debug, Default)]
pub struct SyntaxInventory {
    pub operations: Vec<String>,
    pub opaque_macros: usize,
    pub complex_closures: usize,
    pub nondelegating_methods: usize,
}

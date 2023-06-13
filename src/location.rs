#[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Hash)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

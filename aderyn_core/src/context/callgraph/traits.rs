/// Trait to support reversing of callgraph. (Because, direct impl is not allowed on Foreign Types)
pub trait Transpose {
    fn reverse(&self) -> Self;
}

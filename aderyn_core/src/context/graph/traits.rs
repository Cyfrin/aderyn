/// Trait to support reversing of callgraph. (Because, direct impl is not allowed on Foreign Types)
pub trait Reverseable {
    fn reverse(&self) -> Self;
}

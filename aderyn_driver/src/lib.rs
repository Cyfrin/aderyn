mod display;
mod interface;
mod runner;

pub mod compile;
pub mod config;
pub mod driver;
pub mod process;

pub use aderyn_core::{ast as core_ast, detect as detection_modules, detect::detector};

pub trait MapOrDefault<T> {
    fn map_or_default<U, F>(self, f: F) -> U
    where
        U: Default,
        F: FnOnce(T) -> U;
}

impl<T> MapOrDefault<T> for Option<T> {
    fn map_or_default<U, F>(self, f: F) -> U
    where
        U: Default,
        F: FnOnce(T) -> U,
    {
        self.map(f).unwrap_or_default()
    }
}

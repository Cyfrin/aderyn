pub mod driver;
pub(crate) mod process_foundry;
pub(crate) mod process_hardhat;
pub(crate) mod virtual_foundry;

pub use aderyn_core::ast as core_ast;
pub use aderyn_core::context;
pub use aderyn_core::detect as detection_modules;
pub use aderyn_core::detect::detector;
pub use process_foundry::with_project_root_at;

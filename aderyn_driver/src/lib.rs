#![feature(result_option_map_or_default)]

mod display;
mod interface;
mod mcp;
mod runner;

pub mod compile;
pub mod config;
pub mod driver;
pub mod process;

pub use aderyn_core::{ast as core_ast, detect as detection_modules, detect::detector};
pub use mcp::SingletonMcpServer;

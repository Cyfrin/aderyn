pub mod ast;
pub mod audit;
pub mod context;
pub mod detect;
pub mod stats;
pub mod test_utils;
pub mod visitor;

// Use this to detect issues (Actively maintained)
pub use detect::entrypoint as report;
pub use report::detect_issues;

// Use this as audit tools (De-prioritized)
pub use audit::entrypoint as audit_tools;

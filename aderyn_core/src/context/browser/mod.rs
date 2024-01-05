use super::loader::ContextLoader;
use crate::ast::NodeID;
use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

mod assignments;
mod binary_checks;

pub use assignments::Assignments;
pub use binary_checks::{BinaryCheckStatement, BinaryChecks};

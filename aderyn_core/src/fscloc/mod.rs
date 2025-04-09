pub mod cloc;
pub mod engine;
pub mod insight;
pub mod token;
pub mod util;

#[derive(Debug, Clone)]
pub struct CodeLines {
    pub info_lines: usize,
    pub actual_first_line: isize,
    pub last_line_has_code: bool,
}

pub mod cloc;
pub mod engine;
pub mod insight;
pub mod token;
pub mod util;

#[derive(Debug, Clone)]
pub struct CodeLines {
    pub info_lines: usize,
    pub actual_first_line: isize,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::fscloc::engine;

    #[test]
    fn test_print_loc() {
        let sol = engine::count_lines_of_code(PathBuf::from("../tests/solfiles").as_path());
        let result = sol.lock().unwrap();
        result
            .iter()
            .for_each(|element| println!("{} - {}", element.0, element.1));
        assert_eq!(*result.get("../tests/solfiles/program.sol").unwrap(), 21);
        assert_eq!(*result.get("../tests/solfiles/program2.sol").unwrap(), 32);
        assert_eq!(*result.get("../tests/solfiles/program3.sol").unwrap(), 6);
    }
}

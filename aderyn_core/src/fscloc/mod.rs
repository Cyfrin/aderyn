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
        let src_filepaths = vec![
            "cloc/HeavilyCommentedContract.sol".to_string(),
            "cloc/AnotherHeavilyCommentedContract.sol".to_string(),
        ];
        let sol = engine::count_lines_of_code(
            PathBuf::from("../tests/contract-playground/src/cloc").as_path(),
            &src_filepaths,
        );
        let result = sol.lock().unwrap();
        result
            .iter()
            .for_each(|element| println!("{} - {}", element.0, element.1));
        assert_eq!(
            *result
                .get("../tests/contract-playground/src/cloc/HeavilyCommentedContract.sol")
                .unwrap(),
            21
        );
        assert_eq!(
            *result
                .get("../tests/contract-playground/src/cloc/AnotherHeavilyCommentedContract.sol")
                .unwrap(),
            32
        );
    }

    #[test]
    fn test_print_loc_specific_file() {
        let src_filepaths = vec!["cloc/HeavilyCommentedContract.sol".to_string()];
        let sol = engine::count_lines_of_code(
            PathBuf::from("../tests/contract-playground/src/cloc").as_path(),
            &src_filepaths,
        );
        let result = sol.lock().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            *result
                .get("../tests/contract-playground/src/cloc/HeavilyCommentedContract.sol")
                .unwrap(),
            21
        );
    }
}

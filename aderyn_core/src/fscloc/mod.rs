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

#[cfg(test)]
mod cloc_tests {
    use std::{collections::HashSet, path::PathBuf, str::FromStr};

    use crate::fscloc::engine;

    #[test]
    fn test_print_loc() {
        let src_filepaths = vec![
            "HeavilyCommentedContract.sol".to_string(),
            "AnotherHeavilyCommentedContract.sol".to_string(),
        ];
        let included = HashSet::from_iter(vec![
            PathBuf::from_str("HeavilyCommentedContract.sol").unwrap(),
            PathBuf::from_str("AnotherHeavilyCommentedContract.sol").unwrap(),
        ]);
        let sol = engine::count_lines_of_code_and_collect_line_numbers_to_ignore(
            PathBuf::from("../tests/contract-playground/src/cloc").as_path(),
            &src_filepaths,
            false,
            &included,
        );
        let result = sol.lock().unwrap();
        result.iter().for_each(|element| println!("{} - {}", element.0, element.1.code));
        assert_eq!(
            result
                .get("../tests/contract-playground/src/cloc/HeavilyCommentedContract.sol")
                .unwrap()
                .code,
            21
        );
        assert_eq!(
            result
                .get("../tests/contract-playground/src/cloc/AnotherHeavilyCommentedContract.sol")
                .unwrap()
                .code,
            32
        );
    }

    #[test]
    fn test_print_loc_specific_file() {
        let src_filepaths = vec!["HeavilyCommentedContract.sol".to_string()];
        let included =
            HashSet::from_iter(vec![PathBuf::from_str("HeavilyCommentedContract.sol").unwrap()]);
        let sol = engine::count_lines_of_code_and_collect_line_numbers_to_ignore(
            PathBuf::from("../tests/contract-playground/src/cloc").as_path(),
            &src_filepaths,
            false,
            &included,
        );
        let result = sol.lock().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(
            result
                .get("../tests/contract-playground/src/cloc/HeavilyCommentedContract.sol")
                .unwrap()
                .code,
            21
        );
    }
}

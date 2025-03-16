#[cfg(test)]
mod project_compiler_grouping_tests {
    use std::{env::set_var, path::PathBuf, str::FromStr};

    use crate::process_auto;

    // Tester function
    fn test_grouping_files_to_compile(
        project_root_str: &str,
        src: &Option<PathBuf>,
        include: &Option<Vec<String>>,
        exclude: &Option<Vec<String>>,
    ) {
        let root_path = PathBuf::from_str(project_root_str).unwrap();
        let source = if src.is_some() {
            Some(src.clone().unwrap().to_string_lossy().to_string())
        } else {
            None
        };

        let contexts = process_auto::with_project_root_at(
            root_path.as_path(),
            &source,
            exclude,
            include,
            false,
        );

        for context in contexts {
            eprintln!("Source Units collected: {}", context.source_units().len());
            assert!(!context.source_units().is_empty());
        }
    }

    #[test]
    fn foundry_nft_f23() {
        let project_root_str = "../tests/foundry-nft-f23";
        let src = &Some(PathBuf::from_str("src/").unwrap());
        test_grouping_files_to_compile(project_root_str, src, &None, &None);
    }

    #[test]
    fn adhoc_solidity_files() {
        let project_root_str = "../tests/adhoc-sol-files";
        test_grouping_files_to_compile(project_root_str, &None, &None, &None);
    }

    #[test]
    fn contract_playground() {
        let project_root_str = "../tests/contract-playground";
        let src = &Some(PathBuf::from_str("src/").unwrap());
        test_grouping_files_to_compile(project_root_str, src, &None, &None);
    }

    #[test]
    fn ccip_develop() {
        let project_root_str = "../tests/ccip-contracts/contracts";
        let src = &Some(PathBuf::from_str("src/v0.8/ccip").unwrap());
        let exclude = &Some(vec!["src/v0.8/ccip/test".to_string()]);
        let include = &Some(vec!["src/v0.8/ccip".to_string()]);
        test_grouping_files_to_compile(project_root_str, src, include, exclude);
    }

    #[test]
    fn test_no_files_found_in_scope_id_detected_by_context_src_filepaths() {
        let contexts = process_auto::with_project_root_at(
            &PathBuf::from("../tests/contract-playground").canonicalize().unwrap(),
            &None,
            &None,
            &Some(vec!["NonExistentFile.sol".to_string()]),
            false,
        );
        assert!(contexts.iter().all(|c| c.src_filepaths.is_empty()));
    }

    //#[test]
    //fn test_compiler_input_returns_empty_vector_when_no_solidity_files_present() {
    //    todo!("../tests/no-sol-files")
    //}
}

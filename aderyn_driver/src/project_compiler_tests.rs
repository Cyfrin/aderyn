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

        assert!(!contexts.is_empty());
        contexts.iter().for_each(|c| {
            assert!(!c.source_units().is_empty());
        });
    }

    #[test]
    fn foundry_nft_f23_only() {
        let project_root_str = "../tests/foundry-nft-f23";
        let src = &Some(PathBuf::from_str("src/").unwrap());
        test_grouping_files_to_compile(project_root_str, src, &None, &None);
    }

    #[test]
    fn foundry_nft_f23_icm() {
        let project_root_str = "../tests/foundry-nft-f23-icm";
        set_var("FOUNDRY_PROFILE", "icm");
        test_grouping_files_to_compile(project_root_str, &None, &None, &None);
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
        set_var("FOUNDRY_PROFILE", "vrfv2plus_coordinator");
        test_grouping_files_to_compile(project_root_str, &None, &None, &None);
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

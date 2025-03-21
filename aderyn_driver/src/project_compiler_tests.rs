#[cfg(test)]
mod project_compiler_grouping_tests {
    use std::{
        path::{Path, PathBuf},
        process::{Command, Stdio},
        str::FromStr,
    };

    use crate::{foundry_compiler_helpers::*, process_auto, read_remappings};
    use cyfrin_foundry_compilers::{utils, CompilerInput, Graph, Solc};

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
        let src = &Some(PathBuf::from_str("src/v0.8/").unwrap());
        test_grouping_files_to_compile(project_root_str, src, &None, &None);
    }

    fn test_grouping_files_to_compile(
        project_root_str: &str,
        src: &Option<PathBuf>,
        scope: &Option<Vec<String>>,
        exclude: &Option<Vec<String>>,
    ) {
        let root = utils::canonicalize(project_root_str).unwrap();

        let solidity_files = get_compiler_input(&root);
        let sources = get_relevant_sources(&root, solidity_files, src, scope, exclude);

        println!("Resolving sources versions by graph ...");
        let (remappings, foundry_compilers_remappings) = get_remappings(&root);
        let project = get_project(&root, foundry_compilers_remappings);

        let graph = Graph::resolve_sources(&project.paths, sources).unwrap();
        let (versions, _) = graph.into_sources_by_version(false).unwrap();

        let sources_by_version = versions.get(&project).unwrap();
        for (solc, value) in sources_by_version {
            println!("Compiling {} files with Solc {}", value.1.len(), value.0);
            let pathbufs = value.1.into_keys().collect::<Vec<_>>();
            let files = get_relevant_pathbufs(&root, &pathbufs, src, scope, exclude);

            assert!(solc.solc.exists());

            let command_result = Command::new(solc.solc.clone())
                .args(remappings.clone())
                .arg("--ast-compact-json")
                .args(
                    files.iter().map(|x| x.strip_prefix(root.clone()).unwrap()).collect::<Vec<_>>(),
                )
                .args(solc.args.clone()) // --allowed-paths <root> for older versions of sol
                .current_dir(root.clone())
                .stdout(Stdio::piped())
                .output();

            match command_result {
                Ok(output) => {
                    let _stdout = String::from_utf8(output.stdout).unwrap();
                    if !output.status.success() {
                        let msg = String::from_utf8(output.stderr).unwrap();
                        eprintln!("stderr = {}", msg);
                        eprintln!("cwd = {}", root.display());
                        print_running_command(
                            solc.solc.to_string_lossy().as_ref(),
                            &remappings,
                            &files,
                            &root,
                        );
                        eprintln!("Error running solc command ^^^");
                        // For now, we do not panic because it will prevent us from analyzing other
                        // contexts which can compile successfully
                    } else {
                        // TODO: Create workspace context from stdout
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    panic!("Error running solc command");
                }
            }
        }
    }

    fn print_running_command(
        solc_bin: &str,
        remappings: &Vec<String>,
        files: &Vec<PathBuf>,
        root: &Path,
    ) {
        let mut command = String::new();
        command.push_str(solc_bin);
        command.push_str(" --ast-compact-json ");
        for remap in remappings {
            command.push_str(&format!("{} ", remap));
        }
        for file in files {
            command.push_str(&format!("{} ", file.strip_prefix(root).unwrap().to_string_lossy()));
        }
        eprintln!("{}", command);
    }

    #[test]
    fn directly_solc_and_check_for_ccip() {
        let solc = Solc::find_or_install_svm_version("0.8.16").unwrap();
        let root = utils::canonicalize("../tests/ccip-contracts/contracts").unwrap();

        let mut remappings = vec![];
        if let Some(custom_remappings) = read_remappings(&root) {
            remappings.extend(custom_remappings);
            remappings.dedup();
        }

        let command_result = Command::new(solc.solc)
            .args(remappings.clone())
            .arg("--ast-compact-json")
            .args([
                "src/v0.8/automation/AutomationForwarder.sol",
                "src/v0.8/automation/Chainable.sol",
                "src/v0.8/automation/ExecutionPrevention.sol",
                "src/v0.8/automation/UpkeepFormat.sol",
                "src/v0.8/automation/interfaces/AutomationCompatibleInterface.sol",
                "src/v0.8/automation/interfaces/IAutomationForwarder.sol",
            ])
            .current_dir(root.clone())
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");

        assert!(command_result.status.success());
    }

    #[test]
    fn directly_solc_and_check_for_foundry_nft_f23() {
        let solc = Solc::find_or_install_svm_version("0.8.25").unwrap();
        let root = utils::canonicalize("../tests/foundry-nft-f23").unwrap();

        let mut remappings = vec![];
        if let Some(custom_remappings) = read_remappings(&root) {
            remappings.extend(custom_remappings);
            remappings.dedup();
        }

        let command_result = Command::new(solc.solc)
            .args(remappings.clone())
            .arg("--ast-compact-json")
            .args(["src/BasicNft.sol", "src/inner-core-modules/ICM.sol", "src/Initializer.sol"])
            .current_dir(root.clone())
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");

        println!("{:?}", String::from_utf8(command_result.stderr));

        assert!(command_result.status.success());
    }

    #[test]
    fn test_no_files_found_in_scope_id_detected_by_context_src_filepaths() {
        let contexts = process_auto::with_project_root_at(
            &PathBuf::from("../tests/contract-playground").canonicalize().unwrap(),
            &None,
            &None,
            &None,
            &Some(vec!["NonExistentFile.sol".to_string()]),
            false,
        );
        assert!(contexts.iter().all(|c| c.src_filepaths.is_empty()));
    }

    #[test]
    fn test_compiler_input_returns_empty_vector_when_no_solidity_files_present() {
        let compiler_input = CompilerInput::new("../tests/no-sol-files").unwrap();
        let solidity_files =
            compiler_input.into_iter().filter(|c| c.language == *"Solidity").collect::<Vec<_>>();
        assert!(solidity_files.is_empty());
    }
}

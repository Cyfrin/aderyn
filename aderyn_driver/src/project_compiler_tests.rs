#[cfg(test)]
mod project_compiler_grouping_tests {
    use std::{
        collections::BTreeMap,
        path::PathBuf,
        process::{Command, Stdio},
    };

    use crate::{passes_exclude, passes_scope, read_remappings};
    use foundry_compilers::{utils, CompilerInput, Graph, Project, ProjectPathsConfig, Solc};

    #[test]
    fn foundry_nft_f23() {
        let project_root_str = "../tests/foundry-nft-f23";
        let scope = &Some(vec!["src/".to_string()]);
        let exclude = &Some(vec!["lib/".to_string()]);
        test_grouping_files_to_compile(project_root_str, scope, exclude);
    }

    #[test]
    fn adhoc_solidity_files() {
        let project_root_str = "../tests/adhoc-sol-files";
        test_grouping_files_to_compile(project_root_str, &None, &None);
    }

    #[test]
    fn contract_playground() {
        let project_root_str = "../tests/contract-playground";
        let scope = &Some(vec!["src/".to_string()]);
        let exclude = &Some(vec!["lib/".to_string()]);
        test_grouping_files_to_compile(project_root_str, scope, exclude);
    }

    #[test]
    fn ccip() {
        let project_root_str = "../tests/ccip/contracts";
        let scope = &Some(vec!["src/v0.8/".to_string()]);
        let exclude = &Some(vec![
            "tests/".to_string(),
            "test/".to_string(),
            "testhelpers/".to_string(),
            "lib/".to_string(),
            "node_modules/".to_string(),
            "mocks/".to_string(),
            "vendor/".to_string(),
        ]);
        test_grouping_files_to_compile(project_root_str, scope, exclude);
    }

    fn test_grouping_files_to_compile(
        project_root_str: &str,
        scope: &Option<Vec<String>>,
        exclude: &Option<Vec<String>>,
    ) {
        let root = utils::canonicalize(project_root_str).unwrap();

        let compiler_input = CompilerInput::new(&root).unwrap();
        let solidity_files = compiler_input
            .into_iter()
            .filter(|c| c.language == *"Solidity")
            .collect::<Vec<_>>();
        let solidity_files = &solidity_files[0]; // No Yul Support as of now

        let paths = ProjectPathsConfig::builder().root(&root).build().unwrap();
        let project = Project::builder()
            .no_artifacts()
            .paths(paths)
            .ephemeral()
            .build()
            .unwrap();

        println!("Collecting sources in scope");
        let sources = solidity_files
            .sources
            .iter()
            .filter(|(solidity_file, _)| {
                passes_scope(
                    scope,
                    solidity_file.canonicalize().unwrap().as_path(),
                    &root.to_string_lossy().to_string(),
                )
            })
            .filter(|(solidity_file, _)| {
                passes_exclude(
                    exclude,
                    solidity_file.canonicalize().unwrap().as_path(),
                    &root.to_string_lossy().to_string(),
                )
            })
            .map(|(x, y)| (x.to_owned(), y.to_owned()))
            .collect::<BTreeMap<_, _>>();

        // println!("Sources: {:?}", sources.keys().cloned());

        println!("Resolving sources versions by graph ...");
        let graph = Graph::resolve_sources(&project.paths, sources).unwrap();
        let (versions, _) = graph.into_sources_by_version(project.offline).unwrap();

        let mut remappings = vec![];
        if let Some(custom_remappings) = read_remappings(&root) {
            remappings.extend(custom_remappings);
            remappings.dedup();
        }

        let sources_by_version = versions.get(&project).unwrap();
        for (solc, value) in sources_by_version {
            // let version = value.0;
            // let paths = value.1.keys().map(|x| x.display()).collect::<Vec<_>>();
            // println!("{} - \n{:?}\n\n", version, paths);
            println!("Compiling {} files with Solc {}", value.1.len(), value.0);
            let solc_bin = solc.solc.to_str().unwrap();
            let files: Vec<_> = value
                .1
                .into_keys()
                .filter(|solidity_file| {
                    passes_scope(
                        scope,
                        solidity_file.canonicalize().unwrap().as_path(),
                        &root.to_string_lossy().to_string(),
                    )
                })
                .filter(|solidity_file| {
                    passes_exclude(
                        exclude,
                        solidity_file.canonicalize().unwrap().as_path(),
                        &root.to_string_lossy().to_string(),
                    )
                })
                .collect();

            // println!("Running the following command: ");
            // print_running_command(solc_bin, &remappings, &files, &root);

            // Make sure the solc binary is available
            assert!(solc.solc.exists());

            let command_result = Command::new(solc.solc.clone())
                .args(remappings.clone())
                .arg("--ast-compact-json")
                .args(
                    files
                        .iter()
                        .map(|x| x.strip_prefix(root.clone()).unwrap())
                        .collect::<Vec<_>>(),
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
                        println!("stderr = {}", msg);
                        println!("cwd = {}", root.display());
                        print_running_command(solc_bin, &remappings, &files, &root);
                        panic!("Error running solc command ^^^");
                    }
                    // TODO: Create workspace context from stdout
                }
                Err(e) => {
                    println!("{:?}", e);
                    panic!("Error running solc command");
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print_running_command(
        solc_bin: &str,
        remappings: &Vec<String>,
        files: &Vec<PathBuf>,
        root: &PathBuf,
    ) {
        let mut command = String::new();
        command.push_str(solc_bin);
        command.push_str(" --ast-compact-json ");
        for remap in remappings {
            command.push_str(&format!("{} ", remap));
        }
        for file in files {
            command.push_str(&format!(
                "{} ",
                file.strip_prefix(root.clone())
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            ));
        }
        println!("{}", command);
    }

    #[test]
    fn directly_solc_and_check_for_ccip() {
        let solc = Solc::find_or_install_svm_version("0.8.16").unwrap();
        let root = utils::canonicalize("../tests/ccip/contracts").unwrap();

        let mut remappings = vec![];
        if let Some(custom_remappings) = read_remappings(&root) {
            remappings.extend(custom_remappings);
            remappings.dedup();
        }
        println!("Remappings {:?}", remappings);
        println!("Root {:?}", root);

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
        println!("Remappings {:?}", remappings);
        println!("Root {:?}", root);

        let command_result = Command::new(solc.solc)
            .args(remappings.clone())
            .arg("--ast-compact-json")
            .args([
                "src/BasicNFT.sol",
                "src/inner-core-modules/ICM.sol",
                "src/Initializer.sol",
            ])
            .current_dir(root.clone())
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");

        assert!(command_result.status.success());
    }
}

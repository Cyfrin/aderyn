use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
};

use aderyn_core::{
    ast::SourceUnit, context::workspace_context::WorkspaceContext, visitor::ast_visitor::Node,
};

use foundry_compilers::{
    remappings::Remapping, utils, CompilerInput, Graph, Project, ProjectPathsConfig,
};

use crate::{passes_exclude, passes_scope, read_remappings};

use crate::ensure_valid_root_path;

pub fn with_project_root_at(
    root_path: &Path,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
) -> Vec<WorkspaceContext> {
    let mut workspace_contexts = vec![];

    let root = utils::canonicalize(root_path).unwrap();

    let compiler_input = CompilerInput::new(&root).unwrap();
    let solidity_files = compiler_input
        .into_iter()
        .filter(|c| c.language == *"Solidity")
        .collect::<Vec<_>>();
    let solidity_files = &solidity_files[0]; // No Yul Support as of now

    let mut remappings = vec![];
    if let Some(custom_remappings) = read_remappings(&root) {
        remappings.extend(custom_remappings);
        remappings.dedup();
    }

    let foundry_compilers_remappings = remappings
        .iter()
        .filter_map(|x| Remapping::from_str(x).ok())
        .collect::<Vec<_>>();

    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .remappings(foundry_compilers_remappings)
        .build()
        .unwrap();
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

    let sources_by_version = versions.get(&project).unwrap();
    for (solc, value) in sources_by_version {
        // let version = value.0;
        // let paths = value.1.keys().map(|x| x.display()).collect::<Vec<_>>();
        // println!("{} - \n{:?}\n\n", version, paths);
        println!("Compiling {} files with Solc {}", value.1.len(), value.0);
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
                let stdout = String::from_utf8(output.stdout).unwrap();
                if !output.status.success() {
                    let msg = String::from_utf8(output.stderr).unwrap();
                    eprintln!("stderr = {}", msg);
                    eprintln!("cwd = {}", root.display());
                    // print_running_command(solc_bin, &remappings, &files, &root);
                    eprintln!("Error running solc command ^^^");
                    // For now, we do not panic because it will prevent us from analyzing other contexts which can compile successfully
                } else {
                    let context =
                        create_workspace_context_from_stdout(stdout, scope, exclude, root_path);
                    workspace_contexts.push(context);
                }
            }
            Err(e) => {
                println!("{:?}", e);
                panic!("Error running solc command");
            }
        }
    }

    workspace_contexts
}

fn create_workspace_context_from_stdout(
    stdout: String,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    root_path: &Path,
) -> WorkspaceContext {
    let absolute_root_path_str = &ensure_valid_root_path(&root_path)
        .to_string_lossy()
        .to_string();

    let mut context = WorkspaceContext::default();
    // dbg!(&stdout)

    let mut pick_next_line = false;
    let mut src_filepaths = vec![];

    for line in stdout.lines() {
        if line.starts_with("======= ") {
            let end_marker = line.find(" =======").unwrap();
            let filepath = &PathBuf::from(&line["======= ".len()..end_marker]);
            if passes_scope(
                scope,
                root_path.join(filepath).canonicalize().unwrap().as_path(),
                absolute_root_path_str,
            ) && passes_exclude(
                exclude,
                root_path.join(filepath).canonicalize().unwrap().as_path(),
                absolute_root_path_str,
            ) {
                src_filepaths.push(filepath.to_string_lossy().to_string());
                pick_next_line = true;
            }
        } else if pick_next_line {
            let ast_content = line.to_string();
            let mut source_unit: SourceUnit = serde_json::from_str(&ast_content).unwrap();
            let filepath = source_unit.absolute_path.as_ref().unwrap();
            source_unit.source = std::fs::read_to_string(&root_path.join(filepath)).ok();
            // dbg!(&filepath);
            source_unit.absolute_path = Some(filepath.to_string());
            // dbg!(&filepath);

            source_unit.accept(&mut context).unwrap_or_else(|err| {
                // Exit with a non-zero exit code
                eprintln!("Error loading AST into WorkspaceContext");
                eprintln!("{:?}", err);
                std::process::exit(1);
            });

            pick_next_line = false;
        }
    }

    // println!("{:#?}", context);
    // println!("New context !");
    context.src_filepaths = src_filepaths;
    context
}

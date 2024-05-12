use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use aderyn_core::{
    ast::SourceUnit, context::workspace_context::WorkspaceContext, visitor::ast_visitor::Node,
};

use foundry_compilers::{utils, Graph};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    get_compiler_input, get_project, get_relevant_pathbufs, get_relevant_sources, get_remappings,
    passes_exclude, passes_scope, passes_src,
};

use crate::ensure_valid_root_path;

pub fn with_project_root_at(
    root_path: &Path,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    src: &Option<Vec<String>>,
) -> Vec<WorkspaceContext> {
    let root = utils::canonicalize(root_path).unwrap();
    let src = src.clone().map(|sources| {
        sources
            .into_iter()
            .map(|source| utils::canonicalize(root.join(source)).unwrap())
            .collect::<Vec<_>>()
    });

    let solidity_files = get_compiler_input(&root);
    let sources = get_relevant_sources(&root, solidity_files, &src, scope, exclude);

    println!("Resolving sources versions by graph ...");
    let (remappings, foundry_compilers_remappings) = get_remappings(&root);
    let project = get_project(&root, foundry_compilers_remappings);

    let graph = Graph::resolve_sources(&project.paths, sources).unwrap();
    let (versions, _) = graph.into_sources_by_version(false).unwrap();

    let sources_by_version = versions.get(&project).unwrap();

    sources_by_version
        .into_par_iter()
        .filter_map(|(solc, value)| {
            println!("Compiling {} files with Solc {}", value.1.len(), value.0);
            let pathbufs = value.1.into_keys().collect::<Vec<_>>();
            let files = get_relevant_pathbufs(&root, &pathbufs, &src, scope, exclude);

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
                .stderr(Stdio::piped())
                .spawn();

            match command_result {
                Ok(child) => {
                    let output = child.wait_with_output().unwrap();
                    let stdout = String::from_utf8(output.stdout).unwrap();
                    if !output.status.success() {
                        let msg = String::from_utf8(output.stderr).unwrap();
                        eprintln!("stderr = {}", msg);
                        eprintln!("cwd = {}", root.display());
                        // print_running_command(solc_bin, &remappings, &files, &root);
                        eprintln!("Error running solc command ^^^");
                        // For now, we do not panic because it will prevent us from analyzing other contexts which can compile successfully
                    } else {
                        let context = create_workspace_context_from_stdout(
                            stdout, &src, scope, exclude, root_path,
                        );
                        return Some(context);
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    panic!("Error running solc command");
                }
            }

            None
        })
        .collect()
}

fn create_workspace_context_from_stdout(
    stdout: String,
    src: &Option<Vec<PathBuf>>,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    root_path: &Path,
) -> WorkspaceContext {
    let absolute_root_path_str = &ensure_valid_root_path(root_path)
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
            ) && passes_src(
                src,
                root_path.join(filepath).canonicalize().unwrap().as_path(),
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

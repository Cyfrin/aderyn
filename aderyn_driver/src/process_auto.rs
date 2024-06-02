use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use aderyn_core::{
    ast::SourceUnit, context::workspace_context::WorkspaceContext, visitor::ast_visitor::Node,
};

use cyfrin_foundry_compilers::{utils, Graph};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    get_compiler_input, get_fc_remappings, get_project, get_relevant_pathbufs,
    get_relevant_sources, get_remappings, passes_exclude, passes_scope, passes_src,
};

use crate::ensure_valid_root_path;

pub fn with_project_root_at(
    root_path: &Path,
    src: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    remappings: &Option<Vec<String>>,
    scope: &Option<Vec<String>>,
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
    let (remappings, foundry_compilers_remappings) = {
        match remappings {
            None => get_remappings(&root),
            Some(remappings) => (remappings.clone(), get_fc_remappings(remappings)),
        }
    };
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

    // let mut pick_next_line = false;
    let mut src_filepaths = vec![];

    let lines = stdout.lines().collect::<Vec<_>>();

    let mut idx = 0;
    let mut keep_picking = false;
    let mut ast_content = String::new();

    while idx < lines.len() {
        let line = lines[idx];

        let (separation, filename) =
            is_demarcation_line(line, scope, exclude, root_path, src, absolute_root_path_str);

        if separation {
            if !ast_content.is_empty() {
                absorb_ast_content_into_context(&ast_content, root_path, &mut context);
            }
            ast_content = String::new();
            keep_picking = false;

            if let Some(filepath) = filename {
                src_filepaths.push(filepath);
                keep_picking = true;
            }
        } else if keep_picking {
            ast_content.push_str(line);
        }

        idx += 1;
    }

    if !ast_content.is_empty() {
        absorb_ast_content_into_context(&ast_content, root_path, &mut context);
    }

    context.src_filepaths = src_filepaths;
    context
}

fn absorb_ast_content_into_context(
    ast_content: &str,
    root_path: &Path,
    context: &mut WorkspaceContext,
) {
    let mut source_unit: SourceUnit = serde_json::from_str(ast_content).unwrap();
    let filepath = source_unit.absolute_path.as_ref().unwrap();
    source_unit.source = std::fs::read_to_string(root_path.join(filepath)).ok();
    source_unit.absolute_path = Some(filepath.to_string());

    source_unit.accept(context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading AST into WorkspaceContext");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}

fn is_demarcation_line(
    line: &str,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    root_path: &Path,
    src: &Option<Vec<PathBuf>>,
    absolute_root_path_str: &str,
) -> (bool, Option<String>) {
    if line.starts_with("======= ") {
        let end_marker = line.find(" =======").unwrap();
        let filepath = &PathBuf::from(&line["======= ".len()..end_marker]);
        if passes_scope(
            scope,
            utils::canonicalize(root_path.join(filepath))
                .unwrap()
                .as_path(),
            absolute_root_path_str,
        ) && passes_exclude(
            exclude,
            utils::canonicalize(root_path.join(filepath))
                .unwrap()
                .as_path(),
            absolute_root_path_str,
        ) && passes_src(
            src,
            utils::canonicalize(root_path.join(filepath))
                .unwrap()
                .as_path(),
        ) {
            return (true, Some(filepath.to_string_lossy().to_string()));
        }
        return (true, None);
    }
    (false, None)
}

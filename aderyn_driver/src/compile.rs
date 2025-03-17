use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use aderyn_core::{
    ast::SourceUnit, context::workspace_context::WorkspaceContext, visitor::ast_visitor::Node,
};
use foundry_compilers_aletheia::{
    derive_ast_and_evm_info, AstSourceFile, ExcludeConfig, IncludeConfig,
    ProjectConfigInputBuilder, Source, SourcesConfig,
};

pub fn project(
    root_path: &Path,
    src: &Option<String>,
    exclude: &Option<Vec<String>>,
    include: &Option<Vec<String>>,
    lsp_mode: bool,
) -> Vec<WorkspaceContext> {
    let say = |message: &str| {
        if !lsp_mode {
            println!("{}", message);
        }
    };

    let mut project_config_builder = ProjectConfigInputBuilder::new(root_path);

    if let Some(src) = src {
        project_config_builder = project_config_builder.with_sources(SourcesConfig::Specific(
            PathBuf::from_str(src).unwrap_or_else(|_| panic!("{} is not a valid path", src)),
        ));
    }

    if let Some(exclude_containing) = exclude {
        project_config_builder = project_config_builder
            .with_exclude(ExcludeConfig::Specific(exclude_containing.to_vec()));
    }

    if let Some(include_containing) = include {
        project_config_builder = project_config_builder
            .with_include(IncludeConfig::Specific(include_containing.to_vec()));
    }

    let retrieved_info = derive_ast_and_evm_info(&project_config_builder.build().unwrap()).unwrap();

    let mut contexts = vec![];

    let absolute_root_path = std::fs::canonicalize(root_path)
        .unwrap_or_else(|_| panic!("Root path: {:?} is unable to be canonicalized", root_path));

    for ast_info in retrieved_info.versioned_asts {
        let mut context = WorkspaceContext::default();

        let sources = ast_info.sources.0;
        let sources_ast = ast_info.compiler_output.sources;
        let included = ast_info.included_files;

        say(&format!(
            "Compiling {} contracts using solc version: {}",
            sources_ast.len(),
            ast_info.version
        ));

        for cerror in ast_info.compiler_output.errors {
            if cerror.severity.is_error() {
                eprintln!("Compilation Error: {}", cerror);
                std::process::exit(1);
            }
        }

        for (source_path, ast_source_file) in sources_ast {
            if included.contains(&source_path) {
                let content = sources.get(&source_path).cloned().expect("content not found");
                absorb_ast_content_into_context(
                    ast_source_file,
                    &mut context,
                    content,
                    &absolute_root_path,
                );
                let relative_suffix = source_path.strip_prefix(&absolute_root_path).unwrap();
                context.src_filepaths.push(relative_suffix.to_string_lossy().to_string());
            }
        }
        contexts.push(context);
    }

    contexts
}

fn absorb_ast_content_into_context(
    ast_source_file: AstSourceFile,
    context: &mut WorkspaceContext,
    content: Source,
    absolute_root_path: &Path,
) {
    let Some(ast_content) = ast_source_file.ast else {
        eprintln!("Warning: AST not found in output");
        return;
    };

    let Ok(mut source_unit) = serde_json::from_str::<SourceUnit>(&ast_content) else {
        eprintln!("Unable to serialize Source Unit from AST - \n{}\n", &ast_content);
        let error = serde_json::from_str::<SourceUnit>(&ast_content).unwrap_err();
        eprintln!("{:?}", error);
        std::process::exit(1);
    };

    // Set the source
    source_unit.source = Some(content.content.to_string());

    // Adjust the asbolute filepath to be relative
    let filepath = source_unit.absolute_path.as_ref().unwrap();
    let relative_path = PathBuf::from_str(filepath).unwrap();
    let relative_path = relative_path
        .strip_prefix(absolute_root_path)
        .expect("filepath in AST output is not absolute!");

    // Reset absolute path.
    source_unit.absolute_path = Some(relative_path.to_string_lossy().to_string());

    // TODO: Change absolute_path to type Path instead of String so we don't lose any unicode
    // characters (in the minority of cases)

    source_unit.accept(context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading AST into WorkspaceContext");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}

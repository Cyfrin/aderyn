use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use aderyn_core::{
    ast::SourceUnit, context::workspace_context::WorkspaceContext, visitor::ast_visitor::Node,
};
use foundry_compilers_aletheia::{
    derive_ast_and_evm_info, AstSourceFile, ExcludeConfig, IncludeConfig,
    ProjectConfigInputBuilder, SourcesConfig,
};

pub fn with_project_root_at(
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
            PathBuf::from_str(src).expect(&format!("{} is not a valid path", src)),
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

    for ast_info in retrieved_info.versioned_asts {
        let sources = ast_info.compiler_output.sources;
        let included = ast_info.included_files;

        say(&format!(
            "Compiling {} contracts using solc version: {}",
            sources.len(),
            ast_info.version
        ));

        //let compilation_errors = ast_info.compiler_output.errors;
        //if !compilation_errors.is_empty() {
        //    eprintln!("{:?}", compilation_errors);
        //    std::process::exit(1);
        //}

        let mut context = WorkspaceContext::default();

        for (source_path, ast_source_file) in sources {
            if included.contains(&source_path) {
                absorb_ast_content_into_context(ast_source_file, &mut context);
            }
        }

        contexts.push(context);
    }

    contexts
}

fn absorb_ast_content_into_context(ast_source_file: AstSourceFile, context: &mut WorkspaceContext) {
    let Some(ast_content) = ast_source_file.ast else {
        return;
    };

    let Ok(source_unit) = serde_json::from_str::<SourceUnit>(&ast_content) else {
        eprintln!("Unable to serialize Source Unit from AST - \n{}\n", &ast_content);
        let error = serde_json::from_str::<SourceUnit>(&ast_content).unwrap_err();
        eprintln!("{:?}", error);
        std::process::exit(1);
    };

    //let filepath = source_unit.absolute_path.as_ref().unwrap();

    source_unit.accept(context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading AST into WorkspaceContext");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}

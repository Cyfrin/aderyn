use aderyn_core::{
    ast::SourceUnit, context::workspace::WorkspaceContext, visitor::ast_visitor::Node,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use solidity_ast::{
    derive_ast_and_evm_info, AstSourceFile, ExcludeConfig, IncludeConfig,
    ProjectConfigInputBuilder, Source, SourcesConfig,
};
use std::{path::PathBuf, str::FromStr};

use crate::{
    display::{display_configuration_info, display_header, display_ingesting_message},
    process::PreprocessedConfig,
    MapOrDefault,
};

pub fn project(
    preprocessed_config: PreprocessedConfig,
    lsp_mode: bool,
) -> Result<Vec<WorkspaceContext>, Box<dyn std::error::Error + Sync + Send>> {
    // Decompose pre-processed config
    let PreprocessedConfig { root_path, src, include, exclude } = preprocessed_config;

    // Process the pre-processed config using Cyfrin/solidity-ast-rs to transalate to runtime values
    let path_form_src = |src: &str| -> PathBuf { PathBuf::from_str(src).unwrap() };
    let processed_config = ProjectConfigInputBuilder::new(&root_path)
        .with_sources(src.map_or_default(|src| SourcesConfig::Specific(path_form_src(&src))))
        .with_exclude(exclude.map_or_default(|exclude| ExcludeConfig::Specific(exclude.to_vec())))
        .with_include(include.map_or_default(|include| IncludeConfig::Specific(include.to_vec())))
        .build()?;

    if !lsp_mode {
        display_configuration_info(&processed_config);
        display_header(&processed_config, "Compiling Abstract Syntax Trees");
    }

    // Derive the raw AST content from the source files as per the processed config
    let derived_ast_evm_info = match derive_ast_and_evm_info(&processed_config) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Failed to Derive AST & EVM Info: {}", e);
            return Err("Failed to Derive AST / EVM info".into());
        }
    };

    // Parse the AST content into WorkspaceContexts
    let contexts_results = derived_ast_evm_info
        .versioned_asts
        .into_par_iter() // TODO: Bench to see which is faster - iter() or par_iter()?
        .map(|ast_info| {
            let mut context = WorkspaceContext::default();

            let sources = ast_info.sources.0;
            let sources_ast = ast_info.compiler_output.sources;
            let included = ast_info.included_files;

            for cerror in ast_info.compiler_output.errors {
                if cerror.severity.is_error() {
                    eprintln!("Compilation Error: {}", cerror);
                    return None;
                }
            }

            if !lsp_mode {
                display_ingesting_message(&sources_ast, &included, &ast_info.version.to_string());
            }
            for (source_path, ast_source_file) in sources_ast {
                let content = sources.get(&source_path).expect("content not found");
                absorb_ast_content_into_context(ast_source_file, &mut context, content.clone());
                context.src_filepaths.push(source_path.display().to_string());
            }

            context.evm_version = derived_ast_evm_info.evm_version;
            context.included = included;

            Some(context)
        })
        .collect::<Vec<_>>();

    // Only when not in LSP mode, error out if some context had compilation errors
    if !lsp_mode {
        if contexts_results.iter().any(|c| c.is_none()) {
            std::process::exit(1);
        }
        display_header(&processed_config, "Scanning Contracts");
    }

    // Return the parsed ASTs as a vector of Workspace Contexts
    Ok(contexts_results.into_iter().flatten().collect())
}

fn absorb_ast_content_into_context(
    ast_source_file: AstSourceFile,
    context: &mut WorkspaceContext,
    content: Source,
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

    // Read the relative filepath
    let filepath = source_unit.absolute_path.as_ref().unwrap();

    // Reset absolute path.
    source_unit.absolute_path = Some(filepath.to_string());

    // TODO: Change absolute_path to type Path instead of String so we don't lose any unicode
    // characters (in the minority of cases)

    source_unit.accept(context).unwrap_or_else(|err| {
        // Exit with a non-zero exit code
        eprintln!("Error loading AST into WorkspaceContext");
        eprintln!("{:?}", err);
        std::process::exit(1);
    });
}

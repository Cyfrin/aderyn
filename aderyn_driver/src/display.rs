use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

use solidity_ast::{AstSourceFile, ProjectConfigInput};

pub fn display_header(project_config: &ProjectConfigInput, header: &str) {
    let say_header = |message: &str| {
        let say = |message: &str| {
            println!("{}", message);
        };
        let longest_str_len = project_config.project_paths.sources.display().to_string().len();
        say(&format!("---------{}", &"-".repeat(longest_str_len)));
        say(&format!("# {}", message));
        say(&format!("---------{}", &"-".repeat(longest_str_len)));
    };
    say_header(header);
}

pub fn display_ingesting_message(
    sources_ast: &BTreeMap<PathBuf, AstSourceFile>,
    included: &HashSet<PathBuf>,
    solc_version: &str,
) {
    let ingestion_keys = sources_ast.keys().filter(|&key| included.contains(key)).count();

    if ingestion_keys > 0 {
        println!("Ingesting {} compiled files [solc : v{}]", ingestion_keys, solc_version);
    } else {
        eprintln!("No files found for context [solc : v{}]", solc_version);
    }
}

pub fn display_configuration_info(project_config: &ProjectConfigInput) {
    let say = |message: &str| {
        println!("{}", message);
    };

    let say_header = |message: &str| {
        let longest_str_len = project_config.project_paths.sources.display().to_string().len();
        say(&format!("---------{}", &"-".repeat(longest_str_len)));
        say(&format!("# {}", message));
        say(&format!("---------{}", &"-".repeat(longest_str_len)));
    };

    say("");
    say_header("Configuration");
    say(&format!("Root - {}", project_config.project_paths.root.display()));
    say(&format!("Source - {}", project_config.project_paths.sources.display()));
    say(&format!(
        "Remappings - {:#?}",
        project_config
            .project_paths
            .remappings
            .iter()
            .map(|r| {
                let mut rel = r.clone();
                rel.strip_prefix(&project_config.project_paths.root);
                rel.to_string()
            })
            .collect::<Vec<_>>()
    ));
    say(&format!("EVM version - {}", project_config.evm_version));

    say_header("Source Scope");
    if project_config.include_containing.clone() != vec!["".to_string()] {
        say(&format!("Include Filepaths Containing - {:#?}", project_config.include_containing));
    } else {
        say("Include Filepaths Containing - No specific criteria.");
    }
    if !project_config.exclude_containing.is_empty() {
        say(&format!("Exclude Filepaths Containing - {:#?}", project_config.exclude_containing));
    } else {
        say("Exclude Filepaths Containing - No specific criteria.");
    }

    if !project_config.exclude_starting.is_empty() {
        say(&format!(
            "Auto Excluding Files - {:#?}",
            project_config
                .exclude_starting
                .iter()
                .map(|r| {
                    r.strip_prefix(&project_config.project_paths.sources)
                        .unwrap_or(r)
                        .to_string_lossy()
                        .to_string()
                })
                .collect::<Vec<_>>()
        ));
    } else {
        say("Auto Excluding - No Files.");
    }
}

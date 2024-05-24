use std::{
    fs,
    path::{Path, PathBuf},
};

use foundry_config::Config;
use serde::Deserialize;

/// aderyn.toml structure
#[derive(Deserialize, Clone)]
pub struct AderynConfig {
    pub root: Option<String>,
    pub src: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub remappings: Option<Vec<String>>,
    pub scope: Option<Vec<String>>,
}

/// Load the aderyn.toml file and deserialize it to AderynConfig
fn load_aderyn_config(root: &Path) -> Result<AderynConfig, String> {
    let config_path = root.join("aderyn.toml");
    // Read the file
    let content = fs::read_to_string(config_path)
        .map_err(|err| format!("Error reading config file: {}", err))?;

    // Deserialize the TOML string to AderynConfig
    let config: AderynConfig =
        toml::from_str(&content).map_err(|err| format!("Error parsing config file: {}", err))?;

    Ok(config)
}

#[allow(clippy::type_complexity)]
pub fn derive_from_aderyn_toml(
    root: &PathBuf,
    src: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    remappings: &Option<Vec<String>>,
    scope: &Option<Vec<String>>,
) -> (
    PathBuf,             // Root
    Option<Vec<String>>, // Src
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Remappings
    Option<Vec<String>>, // Scope
) {
    let config = load_aderyn_config(root).unwrap();

    // If the root is the default ".", and the aderyn.toml has a root, use the aderyn.toml root
    let mut local_root: PathBuf = root.clone();
    if let Some(config_root) = &config.root {
        // append the config_root to the local_root
        local_root.push(config_root);
    }

    // If config.src is some, append src if it is not already present
    let mut local_src: Option<Vec<String>> = src.clone();
    if let Some(config_src) = &config.src {
        if let Some(local_src) = &mut local_src {
            if !local_src.contains(config_src) {
                local_src.push(config_src.clone());
            }
        } else {
            local_src = Some(vec![config_src.clone()]);
        }
    }

    // If config.exclude is some, append each value to exclude if it is not already present
    let mut local_exclude = exclude.clone();
    if let Some(config_exclude) = &config.exclude {
        if let Some(local_exclude) = &mut local_exclude {
            for item in config_exclude {
                if !local_exclude.contains(item) {
                    local_exclude.push(item.clone());
                }
            }
        } else {
            local_exclude = Some(config_exclude.clone());
        }
    }

    // If config.remappings is some, append each value to remappings if it is not already present
    let mut local_remappings = remappings.clone();
    if let Some(config_remappings) = &config.remappings {
        if let Some(local_remappings) = &mut local_remappings {
            for item in config_remappings {
                if !local_remappings.contains(item) {
                    local_remappings.push(item.clone());
                }
            }
        } else {
            local_remappings = Some(config_remappings.clone());
        }
    }

    // If config.scope is some, append each value to scope if it is not already present
    let mut local_scope = scope.clone();
    if let Some(config_scope) = &config.scope {
        if let Some(local_scope) = &mut local_scope {
            for item in config_scope {
                if !local_scope.contains(item) {
                    local_scope.push(item.clone());
                }
            }
        } else {
            local_scope = Some(config_scope.clone());
        }
    }

    (
        local_root,
        local_src,
        local_exclude,
        local_remappings,
        local_scope,
    )
}

/// Append the src, remappings, and exclude from the foundry.toml file.
/// If the src and exclude are provided, they will be used instead of the foundry.toml.
/// Otherwise, the foundry.toml will be used.
#[allow(clippy::type_complexity)]
pub fn append_from_foundry_toml(
    root: &Path,
    src: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    remappings: &Option<Vec<String>>,
) -> (
    Option<Vec<String>>, // Src
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Remappings
) {
    let config = Config::load_with_root(root);

    // src
    let mut local_src = src.clone();
    match local_src {
        Some(_) => (),
        None => {
            local_src = Some(vec![config.src.to_string_lossy().to_string()]);
        }
    }

    // exclude
    let mut local_exclude = exclude.clone();
    let script = format!("{}/", config.script.to_string_lossy());
    let test = format!("{}/", config.test.to_string_lossy());
    let libs = config
        .libs
        .iter()
        .map(|x| format!("{}/", x.to_string_lossy()))
        .collect::<Vec<_>>();
    if let Some(local_exclude) = &mut local_exclude {
        local_exclude.push(test);
        local_exclude.push(script);
        local_exclude.extend(libs);
    } else {
        let mut exclude = vec![];
        exclude.push(test);
        exclude.push(script);
        exclude.extend(libs);
        local_exclude = Some(exclude);
    }

    // remappings
    let mut local_remappings = remappings.clone();
    if let Some(local_remappings) = &mut local_remappings {
        local_remappings.extend(
            config
                .get_all_remappings()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
        );
    } else {
        local_remappings = Some(
            config
                .get_all_remappings()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
        );
    }

    (local_src, local_exclude, local_remappings)
}

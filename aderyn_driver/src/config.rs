use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::process::PreprocessedConfig;

/// `aderyn.toml` file structure
#[derive(Deserialize, Clone)]
pub struct AderynConfig {
    pub version: usize,
    pub root: Option<String>,
    pub src: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

pub fn supplement_values_from_aderyn_toml(
    current: PreprocessedConfig,
) -> Result<PreprocessedConfig, Box<dyn std::error::Error + Send + Sync>> {
    let root_path = current.root_path.clone();
    Ok(supplement(current, aderyn_toml_config(&root_path)?))
}

/// Load the aderyn.toml file and deserialize it to AderynConfig
fn aderyn_toml_config(root: &Path) -> Result<AderynConfig, String> {
    let config_path = root.join("aderyn.toml");
    // Read the file
    let content = fs::read_to_string(config_path)
        .map_err(|err| format!("Error reading config file: {}", err))?;

    // Deserialize the TOML string to AderynConfig
    let mut config: AderynConfig =
        toml::from_str(&content).map_err(|err| format!("Error parsing config file: {}", err))?;

    if config.version != 1 {
        return Err("aderyn.toml version not supported".to_owned());
    }

    // Clear empty vectors
    clear_empty_vectors(&mut config.exclude);
    clear_empty_vectors(&mut config.include);

    Ok(config)
}

fn supplement(current: PreprocessedConfig, config: AderynConfig) -> PreprocessedConfig {
    // Load env variables
    if let Some(map) = config.env.clone() {
        map.iter().for_each(|(k, v)| {
            env::set_var(k, v);
        })
    }

    let mut local_root: PathBuf = current.root_path;
    if let Some(config_root) = &config.root {
        // append the config_root to the local_root
        local_root.push(config_root);
    }

    // If config.src is some, command line arg src overrides config.src
    let mut local_src: Option<String> = current.src.clone();
    if let Some(config_src) = &config.src {
        if local_src.is_none() {
            local_src = Some(config_src.clone());
        }
    }

    // If config.exclude is some, append each value to exclude if it is not already present
    let mut local_exclude = current.exclude.clone();
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

    // If config.include is some, append each value to include if it is not already present
    let mut local_include = current.include.clone();
    if let Some(config_scope) = &config.include {
        if let Some(local_include) = &mut local_include {
            for item in config_scope {
                if !local_include.contains(item) {
                    local_include.push(item.clone());
                }
            }
        } else {
            local_include = Some(config_scope.clone());
        }
    }

    PreprocessedConfig {
        root_path: local_root,
        src: local_src,
        exclude: local_exclude,
        include: local_include,
    }
}

fn clear_empty_vectors<T>(vec: &mut Option<Vec<T>>) {
    if let Some(v) = vec {
        if v.is_empty() {
            *vec = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use crate::process::PreprocessedConfig;

    #[test]
    fn test_interpret_aderyn_config_correctly_appends_and_replaces() {
        // Act
        let current = {
            let root = std::path::Path::new("ARG_ROOT");
            let src = Some("ARG_SRC".to_string());
            let exclude = Some(vec!["ARG_EXCLUDE_1".to_string(), "ARG_EXCLUDE_2".to_string()]);
            let include = Some(vec!["ARG_SCOPE_1".to_string(), "ARG_SCOPE_2".to_string()]);
            PreprocessedConfig { root_path: root.to_path_buf(), src, include, exclude }
        };
        let result = {
            let env = HashMap::from_iter(vec![(
                "FOUNDRY_PROFILE".to_string(),
                "ENV_VAR_VALUE".to_string(),
            )]);
            let config = super::AderynConfig {
                version: 1,
                root: Some("CONFIG_ROOT".to_string()),
                src: Some("CONFIG_SRC".to_string()),
                exclude: Some(vec!["CONFIG_EXCLUDE".to_string()]),
                include: Some(vec!["CONFIG_SCOPE".to_string()]),
                env: Some(env),
            };
            super::supplement(current, config)
        };

        // Assert
        assert_eq!(env::var("FOUNDRY_PROFILE").unwrap(), "ENV_VAR_VALUE");
        assert_eq!(result.root_path, std::path::Path::new("ARG_ROOT/CONFIG_ROOT"));
        assert_eq!(result.src, Some("ARG_SRC".to_string()));
        assert_eq!(
            result.exclude,
            Some(vec![
                "ARG_EXCLUDE_1".to_string(),
                "ARG_EXCLUDE_2".to_string(),
                "CONFIG_EXCLUDE".to_string()
            ])
        );
        assert_eq!(
            result.include,
            Some(vec![
                "ARG_SCOPE_1".to_string(),
                "ARG_SCOPE_2".to_string(),
                "CONFIG_SCOPE".to_string()
            ])
        );
    }

    #[test]
    fn test_clear_empty_vectors() {
        let mut vec_1 = Some(vec!["a".to_string(), "b".to_string()]);
        super::clear_empty_vectors(&mut vec_1);
        assert_eq!(vec_1, Some(vec!["a".to_string(), "b".to_string()]));

        let mut vec_2: Option<Vec<String>> = Some(vec![]);
        super::clear_empty_vectors(&mut vec_2);
        assert_eq!(vec_2, None);
    }
}

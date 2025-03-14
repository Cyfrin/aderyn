use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use cyfrin_foundry_config::Config;
use serde::Deserialize;

/// aderyn.toml structure
#[derive(Deserialize, Clone)]
pub struct AderynConfig {
    /// By default we'll assume it's version 1
    pub version: usize,
    pub root: Option<String>,
    pub src: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

/// Load the aderyn.toml file and deserialize it to AderynConfig
fn load_aderyn_config(root: &Path) -> Result<AderynConfig, String> {
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

    load_env_variables_from_config(&config);

    Ok(config)
}

fn load_env_variables_from_config(config: &AderynConfig) {
    // Load env variables
    if let Some(map) = config.env.clone() {
        map.iter().for_each(|(k, v)| {
            env::set_var(k, v);
        })
    }
}

fn clear_empty_vectors<T>(vec: &mut Option<Vec<T>>) {
    if let Some(v) = vec {
        if v.is_empty() {
            *vec = None;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn derive_from_aderyn_toml(
    root: &Path,
    src: &Option<String>,
    exclude: &Option<Vec<String>>,
    remappings: &Option<Vec<String>>,
    include: &Option<Vec<String>>,
) -> (
    PathBuf,             // Root
    Option<String>,      // Src
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Remappings
    Option<Vec<String>>, // Scope
) {
    let config = load_aderyn_config(root).unwrap();
    interpret_aderyn_config(config, root, src, exclude, remappings, include)
}

#[allow(clippy::type_complexity)]
fn interpret_aderyn_config(
    config: AderynConfig,
    root: &Path,
    src: &Option<String>,
    exclude: &Option<Vec<String>>,
    remappings: &Option<Vec<String>>,
    include: &Option<Vec<String>>,
) -> (
    PathBuf,             // Root
    Option<String>,      // Src
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Remappings
    Option<Vec<String>>, // Scope
) {
    let mut local_root: PathBuf = root.to_path_buf();
    if let Some(config_root) = &config.root {
        // append the config_root to the local_root
        local_root.push(config_root);
    }

    // If config.src is some, command line arg src overrides config.src
    let mut local_src: Option<String> = src.clone();
    if let Some(config_src) = &config.src {
        if local_src.is_none() {
            local_src = Some(config_src.clone());
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

    let local_remappings = remappings.clone();

    // If config.include is some, append each value to include if it is not already present
    let mut local_include = include.clone();
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

    (local_root, local_src, local_exclude, local_remappings, local_include)
}

/// Append the src, remappings, and exclude from the foundry.toml file.
/// If the src and exclude are provided, they will be used instead of the foundry.toml.
/// Otherwise, the foundry.toml will be used.
#[allow(clippy::type_complexity)]
pub fn append_from_foundry_toml(
    root: &Path,
    src: &Option<String>,
    exclude: &Option<Vec<String>>,
    remappings: &Option<Vec<String>>,
) -> (
    Option<String>,      // Src
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Remappings
) {
    let config = Config::load_with_root(root);
    interpret_foundry_config(config, src, exclude, remappings)
}

#[allow(clippy::type_complexity)]
fn interpret_foundry_config(
    config: Config,
    src: &Option<String>,
    exclude: &Option<Vec<String>>,
    remappings: &Option<Vec<String>>,
) -> (
    Option<String>,      // Src
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Remappings
) {
    // src
    let mut local_src = src.clone();
    // Only use foundry src if src is not provided
    match local_src {
        Some(_) => (),
        None => {
            local_src = Some(config.src.to_string_lossy().to_string());
        }
    }

    // exclude
    let mut local_exclude = exclude.clone();
    let script = format!("{}/", config.script.to_string_lossy());
    let test = format!("{}/", config.test.to_string_lossy());
    let libs = config.libs.iter().map(|x| format!("{}/", x.to_string_lossy())).collect::<Vec<_>>();
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
        local_remappings
            .extend(config.get_all_remappings().map(|x| x.to_string()).collect::<Vec<_>>());
    } else {
        local_remappings =
            Some(config.get_all_remappings().map(|x| x.to_string()).collect::<Vec<_>>());
    }

    (local_src, local_exclude, local_remappings)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env, path::PathBuf};

    use cyfrin_foundry_config::RelativeRemapping;

    use crate::config_helpers::load_env_variables_from_config;

    #[test]
    fn test_interpret_aderyn_config_correctly_appends_and_replaces() {
        let env =
            HashMap::from_iter(vec![("FOUNDRY_PROFILE".to_string(), "ENV_VAR_VALUE".to_string())]);

        let config = super::AderynConfig {
            version: 1,
            root: Some("CONFIG_ROOT".to_string()),
            src: Some("CONFIG_SRC".to_string()),
            exclude: Some(vec!["CONFIG_EXCLUDE".to_string()]),
            include: Some(vec!["CONFIG_SCOPE".to_string()]),
            env: Some(env),
        };

        let root = std::path::Path::new("ARG_ROOT");
        let src = Some("ARG_SRC".to_string());
        let exclude = Some(vec!["ARG_EXCLUDE_1".to_string(), "ARG_EXCLUDE_2".to_string()]);
        let remappings = Some(vec!["ARG_REMAPPINGS_1".to_string(), "ARG_REMAPPINGS_2".to_string()]);
        let include = Some(vec!["ARG_SCOPE_1".to_string(), "ARG_SCOPE_2".to_string()]);

        load_env_variables_from_config(&config);
        let result =
            super::interpret_aderyn_config(config, root, &src, &exclude, &remappings, &include);
        assert_eq!(result.0, std::path::Path::new("ARG_ROOT/CONFIG_ROOT"));
        assert_eq!(result.1, Some("ARG_SRC".to_string()));
        assert_eq!(
            result.2,
            Some(vec![
                "ARG_EXCLUDE_1".to_string(),
                "ARG_EXCLUDE_2".to_string(),
                "CONFIG_EXCLUDE".to_string()
            ])
        );
        assert_eq!(
            result.3,
            Some(vec!["ARG_REMAPPINGS_1".to_string(), "ARG_REMAPPINGS_2".to_string()])
        );
        assert_eq!(
            result.4,
            Some(vec![
                "ARG_SCOPE_1".to_string(),
                "ARG_SCOPE_2".to_string(),
                "CONFIG_SCOPE".to_string()
            ])
        );

        assert_eq!(env::var("FOUNDRY_PROFILE").unwrap(), "ENV_VAR_VALUE");
    }

    #[test]
    fn test_interpret_foundry_config_correctly_appends_and_replaces() {
        let mut config = cyfrin_foundry_config::Config {
            src: PathBuf::from("CONFIG_SRC"),
            script: PathBuf::from("CONFIG_SCRIPT".to_string()),
            test: PathBuf::from("CONFIG_TEST".to_string()),
            libs: vec![PathBuf::from("CONFIG_LIBS".to_string())],
            ..Default::default()
        };
        let rel_remap = RelativeRemapping {
            context: Some("REL_REMAPPING_CONTEXT".to_string()),
            name: "REL_REMAPPING_NAME".to_string(),
            path: PathBuf::from("REL_REMAPPING_PATH".to_string()).into(),
        };
        config.remappings = vec![rel_remap];

        let src = Some("ADERYN_SRC".to_string());
        let exclude: Option<Vec<String>> =
            Some(vec!["ADERYN_EXCLUDE_1".to_string(), "ADERYN_EXCLUDE_2".to_string()]);
        let remappings =
            Some(vec!["ADERYN_REMAPPINGS_1".to_string(), "ADERYN_REMAPPINGS_2".to_string()]);

        let result = super::interpret_foundry_config(config, &src, &exclude, &remappings);
        assert_eq!(result.0, Some("ADERYN_SRC".to_string()));
        assert_eq!(
            result.1,
            Some(vec![
                "ADERYN_EXCLUDE_1".to_string(),
                "ADERYN_EXCLUDE_2".to_string(),
                "CONFIG_TEST/".to_string(),
                "CONFIG_SCRIPT/".to_string(),
                "CONFIG_LIBS/".to_string(),
            ])
        );
        assert_eq!(
            result.2,
            Some(vec![
                "ADERYN_REMAPPINGS_1".to_string(),
                "ADERYN_REMAPPINGS_2".to_string(),
                "REL_REMAPPING_CONTEXT:REL_REMAPPING_NAME/=REL_REMAPPING_PATH/".to_string()
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

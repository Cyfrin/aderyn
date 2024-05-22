use std::path::Path;

use foundry_config::Config;

/// Derive the src, remappings, and exclude from the foundry.toml file.
/// If the src and exclude are provided, they will be used instead of the foundry.toml.
/// Otherwise, the foundry.toml will be used.
#[allow(clippy::type_complexity)]
pub fn derive_from_foundry_toml(
    root: &Path,
    src: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
) -> (
    Option<Vec<String>>, // Src
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Remappings
) {
    let config = Config::load_with_root(root);

    // src
    let src = match src {
        Some(src) => src.clone(),
        None => {
            vec![config.src.to_string_lossy().to_string()]
        }
    };

    // exclude
    let script = config.script.to_string_lossy().to_string();
    let test = config.test.to_string_lossy().to_string();
    let libs = config.libraries.clone();
    let exclude = match exclude {
        Some(exclude) => exclude.clone(),
        None => {
            let mut exclude = vec![];
            exclude.push(test);
            exclude.push(script);
            exclude.extend(libs);
            exclude
        }
    };

    // remappings
    let remappings = config
        .get_all_remappings()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    (Some(src), Some(exclude), Some(remappings))
}

use std::path::Path;

use foundry_config::Config;

#[allow(clippy::type_complexity)]
pub fn derive_from_foundry_toml(
    root: &Path,
    scope: &Option<Vec<String>>,
    exclude: &Option<Vec<String>>,
    src: &Option<Vec<String>>,
) -> (
    Option<Vec<String>>, // Scope
    Option<Vec<String>>, // Exclude
    Option<Vec<String>>, // Src
    Option<Vec<String>>, // Remappings
) {
    let config = Config::load_with_root(root);

    let libs = config.libraries.clone();
    let test = config.test.to_string_lossy().to_string();

    let remappings = config
        .get_all_remappings()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let src = match src {
        Some(src) => src.clone(),
        None => {
            vec![config.src.to_string_lossy().to_string()]
        }
    };

    let exclude = match exclude {
        Some(exclude) => exclude.clone(),
        None => {
            let mut exclude = vec![];
            exclude.push(test);
            exclude.extend(libs);
            exclude
        }
    };

    (scope.clone(), Some(exclude), Some(src), Some(remappings))
}

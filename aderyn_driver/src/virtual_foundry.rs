use std::{path::PathBuf, process::Stdio};

pub fn build_isolated_workspace_for_file(solidity_file: &str) -> PathBuf {
    let file = PathBuf::from(solidity_file);

    let forge_folder_name = file.file_stem().unwrap().to_str().unwrap();
    let safe_space = get_or_create_safespace(forge_folder_name);

    let new_name = safe_space.join("src").join(file.file_name().unwrap());

    _ = std::fs::remove_file(safe_space.join("src").join("Counter.sol"));
    _ = std::fs::remove_file(safe_space.join("test").join("Counter.t.sol"));
    _ = std::fs::remove_file(safe_space.join("script").join("Counter.s.sol"));

    std::fs::copy(solidity_file, new_name).unwrap_or_else(|x| {
        eprint!("Unable to copy your file to safespace! {}", x);
        std::process::exit(1);
    });

    safe_space
}

pub fn delete_safe_space(folder: &PathBuf) {
    let _ = std::fs::remove_dir_all(folder);
}

pub fn get_or_create_safespace(folder_name: &str) -> PathBuf {
    let config_home_loc = std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();

    let mut config_loc = PathBuf::from(config_home_loc);
    config_loc.push("aderyn");

    if !config_loc.exists() {
        std::fs::create_dir_all(config_loc.as_path()).expect(
            "Couldn't initialize aderyn folder at home directory for analyzing vulenrabilities ",
        );
    }

    // Run `forge init <folder_name>` in the root
    let _output = std::process::Command::new("forge")
        .args(["init", folder_name])
        .current_dir(&config_loc)
        .stdout(Stdio::inherit()) // This will stream the stdout
        .stderr(Stdio::inherit())
        .status();

    config_loc.join(folder_name)
}

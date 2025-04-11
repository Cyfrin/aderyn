use xshell::{Shell, cmd};

pub fn init_toml_files() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(env!("CARGO_MANIFEST_DIR"));
    sh.change_dir("../../");

    let cmd = cmd!(sh, "rm -rf ./tests/toml/nested_project1/aderyn.toml");
    cmd.run()?;

    let cmd = cmd!(sh, "rm -rf ./tests/toml/nested_project2/aderyn.toml");
    cmd.run()?;

    let cmd = cmd!(sh, "cargo run -- init ./tests/toml/nested_project1");
    cmd.run()?;

    let cmd = cmd!(sh, "cargo run -- init ./tests/toml/nested_project2");
    cmd.run()?;

    Ok(())
}

use xshell::{Shell, cmd};

pub fn blesspr() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(env!("CARGO_MANIFEST_DIR"));
    sh.change_dir("../../");

    // Run cargo build (sanity test)
    let cmd = cmd!(sh, "cargo build");
    cmd.run()?;

    // Fix clippy
    let cmd = cmd!(sh, "cargo fixclippy");
    cmd.run()?;

    // Check clippy
    let cmd = cmd!(sh, "cargo clippy").arg("--").arg("-D").arg("warnings");
    cmd.run()?;

    // Fix format
    let cmd = cmd!(sh, "cargo fixfmt");
    cmd.run()?;

    // Check fixed format
    let cmd = cmd!(sh, "cargo fmt --all --check");
    cmd.run()?;

    // Create reportgen
    let cmd = cmd!(sh, "cargo prep --all --parallel");
    cmd.run()?;

    // Create aderyn.toml
    let cmd = cmd!(sh, "cargo tomlgen");
    cmd.run()?;

    // Push changes
    let cmd = cmd!(sh, "git add .");
    cmd.run()?;
    let cmd = cmd!(sh, "git commit -am").arg("chore: cargo blesspr");
    cmd.run()?;
    let cmd = cmd!(sh, "git config push.autoSetupRemote true");
    cmd.run()?;
    let cmd = cmd!(sh, "git push");
    cmd.run()?;

    Ok(())
}

use xshell::{Shell, cmd};

pub fn fixpr() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(env!("CARGO_MANIFEST_DIR"));
    sh.change_dir("../../");

    // Run cargo build (sanity test)
    let cmd = cmd!(sh, "cargo build");
    cmd.run()?;

    // Fix format
    let cmd = cmd!(sh, "cargo +nightly fmt --all");
    cmd.run()?;

    // Check fixed format
    let cmd = cmd!(sh, "cargo +nightly fmt --all --check");
    cmd.run()?;

    // Fix clippy
    let cmd = cmd!(
        sh,
        "cargo clippy --quiet --workspace --all-targets --all-features --allow-dirty --fix"
    );
    cmd.run()?;

    // Create reportgen
    let cmd = cmd!(sh, "chmod +x ./cli/reportgen.sh");
    cmd.run()?;
    let cmd = cmd!(sh, "./cli/reportgen.sh");
    cmd.run()?;

    // Push changes
    let cmd = cmd!(
        sh,
        "git add . && git commit -am \"chore: cargo fixpr\" && git
     push"
    );
    cmd.run()?;

    Ok(())
}

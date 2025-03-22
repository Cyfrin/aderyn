use xshell::{Shell, cmd};

pub fn fixpr() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let cmd = cmd!(sh, "make pr");
    cmd.run()?;
    Ok(())
}

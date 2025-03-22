use std::io::BufRead;

use xshell::{Shell, cmd};

use crate::flags::CutRelease;

pub fn cut_release(cut_release: CutRelease) -> anyhow::Result<()> {
    let sh = Shell::new()?;

    // Sanity checks and syncs
    sync_tags(&sh)?;
    // TODO:
    //perform_prechecks(&sh)?;

    // Release process
    if cut_release.execute {
        dry_run(&sh, &cut_release)?;
        kick_off_release(&sh, &cut_release)?;
    } else {
        dry_run(&sh, &cut_release)?;
        println!("If everything looks good, rerun with `--execute` flag!");
    }

    Ok(())
}

fn kick_off_release(sh: &Shell, cut_release: &CutRelease) -> anyhow::Result<()> {
    let execute_cmd = if cut_release.patch {
        cmd!(sh, "cargo release patch --no-publish --exclude xtask --execute")
    } else if cut_release.minor {
        cmd!(sh, "cargo release minor --no-publish --exclude xtask --execute")
    } else {
        unreachable!()
    };

    println!("Kick off the release process\n\taderyn\n?[y/N]");
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    if line.contains("y") {
        println!("Kicked-off release process!");
        let d = execute_cmd.stdin(line.clone());
        d.run()?;
    } else {
        println!("Declined release process!");
    }

    Ok(())
}

fn dry_run(sh: &Shell, cut_release: &CutRelease) -> anyhow::Result<()> {
    let dry_run_command = if cut_release.patch {
        cmd!(sh, "cargo release patch --no-publish --exclude xtask --no-tag")
    } else if cut_release.minor {
        cmd!(sh, "cargo release minor --no-publish --exclude xtask --no-tag")
    } else {
        unreachable!();
    };
    dry_run_command.run()?;
    Ok(())
}

fn sync_tags(sh: &Shell) -> anyhow::Result<()> {
    let sync = cmd!(sh, "git fetch --prune-tags origin");
    sync.run()?;
    Ok(())
}

fn perform_prechecks(sh: &Shell) -> anyhow::Result<()> {
    // Exit if not on dev branch
    let curr_branch = {
        let cmd = cmd!(sh, "git rev-parse --abbrev-ref HEAD");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    if curr_branch != "dev" {
        eprintln!("Please switch to dev branch and retry!");
        std::process::exit(1);
    }

    // Error out if there are untracked files/staging changes
    let uncommited_stuff = {
        let cmd = cmd!(sh, "git status --porcelain");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    if !uncommited_stuff.is_empty() {
        eprintln!("Please clear your staging area and retry!");
        std::process::exit(1);
    }

    // Error if dev branch is not in sync with remote
    let local_commit_hash = {
        let cmd = cmd!(sh, "git rev-parse dev");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    let remote_commit_hash = {
        let cmd = cmd!(sh, "git rev-parse dev");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    if remote_commit_hash != local_commit_hash {
        eprintln!("dev branch is not in sync with origin. Do that and retry!");
        std::process::exit(1);
    }
    Ok(())
}
